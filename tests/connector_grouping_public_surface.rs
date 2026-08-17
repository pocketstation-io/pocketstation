use pocketstation::{
    EndpointDriverFactory, EndpointFailure, EndpointGroupId, EndpointPortInput,
    EndpointPreparationGroup, OperatorConfiguration, PreparedEndpointDriver, RouteId,
};

struct ExternalConnectorFactory;

impl EndpointDriverFactory for ExternalConnectorFactory {
    fn preparation_group(
        &self,
        _route_id: RouteId,
        _configuration: &OperatorConfiguration,
    ) -> Result<EndpointPreparationGroup, EndpointFailure> {
        Ok(EndpointPreparationGroup::Shared(EndpointGroupId::new(
            "external-connector-group",
        )))
    }

    fn prepare(
        &self,
        _inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        unreachable!("the public grouping regression does not prepare media")
    }
}

#[test]
fn given_external_factory_when_grouping_routes_then_public_shared_group_is_addressable() {
    let group = ExternalConnectorFactory
        .preparation_group(RouteId::new(1), &OperatorConfiguration::new())
        .expect("public preparation group");

    assert_eq!(
        group,
        EndpointPreparationGroup::Shared(EndpointGroupId::new("external-connector-group"))
    );
}
