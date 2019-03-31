#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use bytes::Bytes;
use futures::Future;
use hashbrown::HashMap;
use interledger_ildcp::IldcpAccount;
use interledger_service::Account;

#[cfg(test)]
mod fixtures;
mod packet;
mod routing_table;
mod server;
#[cfg(test)]
mod test_helpers;

pub use server::CcpServerService;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum RoutingRelation {
    Parent = 1,
    Peer = 2,
    Child = 3,
}

/// DefineCcpAccountethods Account types need to be used by the CCP Service
pub trait RoutingAccount: Account + IldcpAccount {
    /// The type of relationship we have with this account
    fn routing_relation(&self) -> RoutingRelation;

    /// Indicates whether we should send CCP Route Updates to this account
    fn should_send_routes(&self) -> bool {
        false
    }

    /// Indicates whether we should accept CCP Route Update Requests from this account
    fn should_receive_routes(&self) -> bool {
        false
    }
}

pub trait RouteManagerStore: Clone {
    type Account: RoutingAccount;

    // TODO should we have a way to only get the details for specific routes?
    fn get_local_and_configured_routes(
        &self,
    ) -> Box<
        Future<Item = (HashMap<Bytes, Self::Account>, HashMap<Bytes, Self::Account>), Error = ()>
            + Send,
    >;

    fn get_accounts_to_send_route_updates_to(
        &self,
    ) -> Box<Future<Item = Vec<Self::Account>, Error = ()> + Send>;

    fn set_routes(
        &mut self,
        routes: HashMap<Bytes, Self::Account>,
    ) -> Box<Future<Item = (), Error = ()> + Send>;
}
