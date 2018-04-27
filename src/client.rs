use std::fmt;
use std::str;
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::Future;
use futures::future::{
    self,
    Either,
    Loop,
};
use krpc::HostPort;
use krpc;
use parking_lot::Mutex;
use tokio_timer::Delay;

use pb::master::{
    AlterTableResponsePb,
    CreateTableResponsePb,
    DeleteTableRequestPb,
    DeleteTableResponsePb,
    //GetTableSchemaRequestPb,
    //GetTableSchemaResponsePb,
    IsAlterTableDoneRequestPb,
    IsAlterTableDoneResponsePb,
    IsCreateTableDoneRequestPb,
    IsCreateTableDoneResponsePb,
    ListMastersResponsePb,
    ListTablesRequestPb,
    ListTablesResponsePb,
    ListTabletServersResponsePb,
    MasterService,
    TableIdentifierPb,
};
use pb::ExpectField;

use Error;
use IntoMasterAddrs;
use MasterInfo;
use Options;
use Result;
use TableId;
use TabletServerInfo;
use backoff::Backoff;
use master::MasterProxy;
use meta_cache::MetaCache;
use table::AlterTableBuilder;
use table::Table;
use table::TableBuilder;

/// A Kudu database client.
///
/// Encapsulates the connection to a Kudu cluster. Only a single instance should be used per
/// application per cluster.
#[derive(Clone)]
pub struct Client {
    meta_cache: MetaCache,
    latest_observed_timestamp: Arc<Mutex<u64>>, // Replace with AtomicU64 when stable.
}

impl Client {

    /// Creates a new client with the provided configuration.
    fn new<Addrs>(master_addresses: Addrs, options: Options) -> impl Future<Item=Client, Error=Error> 
    where Addrs: IntoMasterAddrs {
        future::result(master_addresses.into_master_addrs())
               .and_then(|master_addresses| MetaCache::new(master_addresses, options))
               .map(|meta_cache| Client {
                   meta_cache,
                   latest_observed_timestamp: Arc::new(Mutex::new(0)),
               })
    }

    /*
    /// Creates a new Kudu table with the schema and options specified by `builder`. Returns the
    /// new table's ID, or an error on failure.
    pub fn create_table(&mut self, builder: TableBuilder) -> impl Future<Item=TableId, Error=Error> {
        let pb = match builder.into_pb().map(Arc::new) {
            Ok(pb) => pb,
            Err(error) => return Either::B(future::err(error)),
        };
        let deadline = self.deadline();
        let call = MasterService::create_table(pb, deadline);

        let mut client = self.clone();
        let response = self.master_proxy()
                           .send(call)
                           .and_then(|response: CreateTableResponsePb| -> Result<TableId> {
                               TableId::parse_bytes(&response.table_id.expect_field("CreateTableResponsePb",
                                                                                    "table_id")?)
                           })
                           .and_then(move |table_id| {
                               client.wait_for_table_creation(table_id)
                                     .map(move |_| table_id)
                           });

        Either::A(response)
    }

    /// Returns a future which completes when the table is created.
    ///
    /// Not on timeout: this method will not timeout if the master is reachable and responsive.
    fn wait_for_table_creation(&mut self, table: TableId) -> impl Future<Item=(), Error=Error> {
        struct State {
            client: Client,
            table: TableId,
            backoff: Backoff,
        }

        let state = State {
            client: self.clone(),
            table,
            backoff: Backoff::with_duration_range(32, 2048),
        };

        future::loop_fn(state, |mut state: State| {
            Delay::new(Instant::now() + state.backoff.next_backoff())
                 .map_err(|error| -> Error { panic!("timer failed: {}", error); })
                 .and_then(move |_| {

                    let call = MasterService::is_create_table_done(
                        Arc::new(IsCreateTableDoneRequestPb { table: state.table.into() }),
                        state.client.deadline());

                    state.client
                         .master_proxy()
                         .send(call)
                         .map(move |response: IsCreateTableDoneResponsePb| {
                             if response.done() {
                                 Loop::Break(())
                             } else {
                                 Loop::Continue(state)
                             }
                         })
                    })
        })
    }

    /// Deletes the table.
    pub fn delete_table<S>(&mut self, table: S) -> impl Future<Item=(), Error=Error>
    where S: Into<String> {
        self.do_delete_table(table.into().into())
    }

    /// Deletes the table.
    pub fn delete_table_by_id(&mut self, id: TableId) -> impl Future<Item=(), Error=Error> {
        self.do_delete_table(id.into())
    }

    fn do_delete_table(&mut self, table: TableIdentifierPb) -> impl Future<Item=(), Error=Error>{
        let call = MasterService::delete_table(Arc::new(DeleteTableRequestPb { table }),
                                               self.deadline());

        self.master_proxy().send(call).map(|_: DeleteTableResponsePb| ())
    }

    pub fn alter_table<S>(&mut self, table: S, alter: AlterTableBuilder) -> impl Future<Item=TableId, Error=Error>
    where S: Into<String> {
        self.do_alter_table(table.into().into(), alter)
    }

    pub fn alter_table_by_id(&mut self, id: TableId, alter: AlterTableBuilder) -> impl Future<Item=(), Error=Error> {
        self.do_alter_table(id.into(), alter).map(|_| ())
    }

    pub fn do_alter_table(&mut self,
                          identifier: TableIdentifierPb,
                          alter: AlterTableBuilder)
                          -> impl Future<Item=TableId, Error=Error> {
        let AlterTableBuilder { result, mut pb, schema } = alter;
        if let Err(error) = result {
            return Either::B(future::err(error));
        }

        pb.table = identifier;
        let call = MasterService::alter_table(Arc::new(pb), self.deadline());
        let client: Client = self.clone();
        let result = self.master_proxy().send(call).and_then(move |resp: AlterTableResponsePb| {
            let table_id = str::from_utf8(resp.table_id())
                               .map_err(|error| Error::Serialization(format!("{}", error)))
                               .and_then(TableId::parse)?;

            // If the table partitioning was altered and there is an existing meta cache for the
            // table, clear it.
            if schema.is_some() {
                // TODO
                // client.meta_cache.clear_table_locations(table_id);
            }

            Ok((table_id, client))
        }).and_then(|(table_id, mut client): (TableId, Client)| {
            client.wait_for_table_alteration(table_id)
                  .map(move |_| table_id)
        });
        Either::A(result)
    }

    /// Returns a future which completes when the table is altered.
    ///
    /// Not on timeout: this method will not timeout if the master is reachable and responsive.
    fn wait_for_table_alteration(&mut self, table: TableId) -> impl Future<Item=(), Error=Error> {
        struct State {
            client: Client,
            table: TableId,
            backoff: Backoff,
        }

        let state = State {
            client: self.clone(),
            table,
            backoff: Backoff::with_duration_range(32, 2048),
        };

        future::loop_fn(state, |mut state: State| {
            Delay::new(Instant::now() + state.backoff.next_backoff())
                 .map_err(|error| -> Error { panic!("timer failed: {}", error); })
                 .and_then(move |_| {

                    let call = MasterService::is_alter_table_done(
                        Arc::new(IsAlterTableDoneRequestPb { table: state.table.into() }),
                        state.client.deadline());

                    state.client
                         .master_proxy()
                         .send(call)
                         .map(move |response: IsAlterTableDoneResponsePb| {
                             if response.done() {
                                 Loop::Break(())
                             } else {
                                 Loop::Continue(state)
                             }
                         })
                    })
        })
    }

    /// Lists all tables and their associated table ID.
    pub fn list_tables(&mut self) -> impl Future<Item=Vec<(String, TableId)>, Error=Error> {
        self.do_list_tables(Default::default())
    }

    /// Lists all tables with the a name matching the provided prefix, and their associated table ID.
    pub fn list_tables_with_prefix<S>(&mut self, name_prefix: S) -> impl Future<Item=Vec<(String, TableId)>, Error=Error>
    where S: Into<String> {
        self.do_list_tables(Arc::new(ListTablesRequestPb { name_filter: Some(name_prefix.into()) }))
    }

    fn do_list_tables(&mut self, request: Arc<ListTablesRequestPb>) -> impl Future<Item=Vec<(String, TableId)>, Error=Error> {
        let call = MasterService::list_tables(request, self.deadline());

        self.master_proxy().send(call).and_then(|response: ListTablesResponsePb| {
            let mut tables = Vec::with_capacity(response.tables.len());
            for table in response.tables {
                tables.push((table.name, TableId::parse_bytes(&table.id)?));
            }
            Ok(tables)
        })
    }

    pub fn list_masters(&mut self) -> impl Future<Item=Vec<MasterInfo>, Error=Error> {
        let call = MasterService::list_masters(Default::default(), self.deadline());

        self.master_proxy().send(call).and_then(|response: ListMastersResponsePb| {
            let mut servers = Vec::with_capacity(response.masters.len());
            for server in response.masters {
                servers.push(MasterInfo::from_pb(server)?);
            }
            Ok(servers)
        })
    }

    pub fn list_tablet_servers(&mut self) -> impl Future<Item=Vec<TabletServerInfo>, Error=Error> {
        let call = MasterService::list_tablet_servers(Default::default(), self.deadline());

        self.master_proxy().send(call).and_then(|response: ListTabletServersResponsePb| {
            let mut servers = Vec::with_capacity(response.servers.len());
            for server in response.servers {
                servers.push(TabletServerInfo::from_pb(server)?);
            }
            Ok(servers)
        })
    }

    /// Returns an open table.
    pub fn open_table<S>(&mut self, table: S) -> impl Future<Item=Table, Error=Error>
    where S: Into<String> {
        self.meta_cache.clone().open_table(TableIdentifierPb::from(table.into()),
                                           self.master_proxy.clone(),
                                           &self.options)
    }

    /// Returns an open table.
    pub fn open_table_by_id(&mut self, id: TableId) -> impl Future<Item=Table, Error=Error> {
        self.meta_cache.clone().open_table(id.into(), self.master_proxy.clone(), &self.options)
    }

    pub fn latest_observed_timestamp(&self) -> u64 {
        *self.latest_observed_timestamp.lock()
    }

    pub fn observe_timestamp(&self, timestamp: u64) {
        let mut latest = self.latest_observed_timestamp.lock();
        if timestamp > *latest {
            *latest = timestamp;
        }
    }

    fn deadline(&self) -> Instant {
        Instant::now() + self.options.admin_timeout
    }

    pub(crate) fn master_proxy(&self) -> MasterProxy {
        self.master_proxy.clone()
    }

    /*
    /// This should only be called when the table has been guaranteed to have been opened.
    pub(crate) fn meta_cache(&self, table: &TableId) -> TableLocationsCache {
        self.meta_caches.lock()[table].clone()
    }
    */
    */
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client")
    }
}

#[cfg(test)]
mod tests {

    use AlterTableBuilder;
    use Column;
    use DataType;
    use RangePartitionBound;
    use SchemaBuilder;
    use TableBuilder;
    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use schema::tests::simple_schema;
    use super::*;
    use util::run;

    use env_logger;
    use futures::sync::oneshot;
    use tokio;

    #[test]
    fn table_lifecycle() {
        let _ = env_logger::init();
        let mut cluster = MiniCluster::default();
        let mut client = ClientBuilder::new(cluster.master_addrs())
                                       .build()
                                       .expect("client");

        let schema = SchemaBuilder::new()
            .add_column(Column::builder("key", DataType::Int32).set_not_null())
            .add_column(Column::builder("val", DataType::Int32))
            .set_primary_key(vec!["key"])
            .build()
            .unwrap();

        let mut table_builder = TableBuilder::new("table_lifecycle", schema.clone());
        table_builder.add_hash_partitions(vec!["key"], 4);
        table_builder.set_num_replicas(1);

        let table_id = run(client.create_table(table_builder)).expect("create_table");
        let mut alter_builder = AlterTableBuilder::new();
        alter_builder.rename_table("table_lifecycle_renamed");
        run(client.alter_table_by_id(table_id, alter_builder)).expect("alter_table_by_id");

        let table = run(client.open_table("table_lifecycle_renamed")).expect("open_table");
        assert_eq!(table_id, table.id());

        let tables = run(client.list_tables()).expect("list_tables");
        assert_eq!(vec![("table_lifecycle_renamed".to_string(), table_id)], tables);

        run(client.delete_table_by_id(table_id)).expect("delete_table");
        assert!(run(client.list_tables()).expect("list_tables").is_empty());
    }

    /*
    #[test]
    fn list_servers() {
        let _ = env_logger::init();
        let mut cluster = MiniCluster::new(MiniClusterConfig::default()
                                                             .num_masters(3)
                                                             .num_tservers(3));
        let mut reactor = Core::new().unwrap();
        let mut client = ClientBuilder::new(cluster.master_addrs())
                                       .build()
                                       .expect("client");

        let tablet_servers = reactor.run(client.list_tablet_servers()).expect("list_table_servers");
        assert_eq!(3, tablet_servers.len());

        let masters = reactor.run(client.list_masters()).expect("list_masters");
        assert_eq!(3, masters.len());
    }

    #[test]
    fn alter_table() {
        let _ = env_logger::init();
        let mut cluster = MiniCluster::new(MiniClusterConfig::default()
                                                             .num_masters(1)
                                                             .num_tservers(1));
        let mut reactor = Core::new().unwrap();
        let mut client = ClientBuilder::new(cluster.master_addrs())
                                       .build()
                                       .expect("client");

        let mut table_builder = TableBuilder::new("t", simple_schema());
        table_builder.set_num_replicas(1);
        table_builder.set_range_partition_columns(vec!["key"]);
        let table_id = reactor.run(client.create_table(table_builder)).expect("create_table");

        let mut alter_builder = AlterTableBuilder::new();
        alter_builder.add_column(Column::builder("c0", DataType::Int32));

        let _ = reactor.run(client.alter_table("t", alter_builder)).expect("add column");

        let schema = reactor.run(client.open_table("t")).expect("open_table").schema().clone();
        assert_eq!(3, schema.columns().len());

        let mut alter_builder = AlterTableBuilder::new();
        alter_builder.drop_range_partition(&RangePartitionBound::Inclusive(schema.new_row()),
                                           &RangePartitionBound::Exclusive(schema.new_row()));

        reactor.run(client.alter_table_by_id(table_id, alter_builder)).expect("drop range partition");

        let mut lower_bound = schema.new_row();
        let mut upper_bound = schema.new_row();

        lower_bound.set_by_name("key", "a").unwrap();
        upper_bound.set_by_name("key", "z").unwrap();

        let mut alter_builder = AlterTableBuilder::new();
        alter_builder.add_range_partition(&RangePartitionBound::Inclusive(lower_bound),
                                          &RangePartitionBound::Exclusive(upper_bound))
                     .rename_table("u")
                     .drop_column("c0");
        reactor.run(client.alter_table_by_id(table_id, alter_builder)).unwrap();

        let schema = reactor.run(client.open_table("u")).unwrap().schema().clone();
        assert_eq!(2, schema.columns().len());
    }
    */
}
