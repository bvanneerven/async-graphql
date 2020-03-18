use crate::{registry, ContextBase, GQLSubscription, GQLType, QueryError, Result};
use graphql_parser::query::Field;
use serde_json::Value;
use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

/// Empty subscription
///
/// Only the parameters used to construct the Schema, representing an unconfigured subscription.
pub struct GQLEmptySubscription;

impl GQLType for GQLEmptySubscription {
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("EmptyMutation")
    }

    fn create_type_info(registry: &mut registry::Registry) -> String {
        registry.create_type::<Self, _>(|_| registry::Type::Object {
            name: "EmptySubscription",
            description: None,
            fields: Default::default(),
        })
    }
}

#[async_trait::async_trait]
impl GQLSubscription for GQLEmptySubscription {
    fn create_type(_field: &Field, _types: &mut HashMap<TypeId, Field>) -> Result<()> {
        return Err(QueryError::NotConfiguredSubscriptions.into());
    }

    async fn resolve(
        &self,
        _ctx: &ContextBase<'_, ()>,
        _types: &HashMap<TypeId, Field, RandomState>,
        _msg: &(dyn Any + Send + Sync),
    ) -> Result<Option<Value>> {
        return Err(QueryError::NotConfiguredSubscriptions.into());
    }
}
