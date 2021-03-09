#[cynic::query_module(
    schema_path = r#"../schemas/schema.graphql"#,
    query_module = "query_dsl",
)]
mod queries {
    use super::{query_dsl, types::*};

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct InsertWeekArguments {
        pub option: Option<String>,
        pub value: Option<String>,
        pub day_title: Option<String>,
        pub week_title: Option<String>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct DeleteWeekArguments {
        pub title: Option<String>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct WeekQueryArguments {
        pub title: Option<String>,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct UpdateWeekArguments {
        pub option: Option<String>,
        pub value: Option<String>,
        pub day_title: Option<String>,
        pub week_title: Option<String>,
        pub query_title: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", argument_struct = "WeekQueryArguments")]
    pub struct WeekQuery {
        #[arguments(query = WeekQueryInput { title: Some(args.title.clone()) })]
        pub week: Option<Week>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Week")]
    pub struct Week {
        pub days: Option<Vec<Option<WeekDay>>>,
        pub title: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "WeekDay")]
    pub struct WeekDay {
        pub meals: Option<Vec<Option<WeekDayMeal>>>,
        pub title: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "WeekDayMeal")]
    pub struct WeekDayMeal {
        pub value: Option<String>,
        pub option: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", argument_struct = "UpdateWeekArguments")]
    pub struct UpdateWeek {
        #[arguments(set = WeekUpdateInput { 
            days: Some(vec![Some(WeekDayUpdateInput { 
                meals: Some(vec![Some(WeekDayMealUpdateInput { 
                    option: Some(args.option.clone()), 
                    value: Some(args.value.clone()) })]), 
                title: Some(args.day_title.clone()) })]), 
            title: Some(args.week_title.clone()) 
        }, query = WeekQueryInput { title: Some(args.query_title.clone()) })]
        pub update_one_week: Option<Week>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", argument_struct = "InsertWeekArguments")]
    pub struct InsertWeek {
        #[arguments(data = WeekInsertInput { 
            days: Some(vec![Some(WeekDayInsertInput { 
                meals: Some(vec![Some(WeekDayMealInsertInput { 
                    option: Some(args.option.clone()), 
                    value: Some(args.value.clone()) })]), 
                title: Some(args.day_title.clone()) })]), 
            title: Some(args.week_title.clone()) })]
        pub insert_one_week: Option<Week>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", argument_struct = "DeleteWeekArguments")]
    pub struct DeleteWeek {
        #[arguments(query = WeekQueryInput { title: Some(args.title.clone()) })]
        pub delete_one_week: Option<DeletedWeek>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Week")]
    pub struct DeletedWeek {
        pub title: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "WeekUpdateInput")]
    pub struct WeekUpdateInput {
        pub days: Option<Vec<Option<WeekDayUpdateInput>>>,
        pub title: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "WeekQueryInput")]
    pub struct WeekQueryInput {
        pub title: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "WeekInsertInput")]
    pub struct WeekInsertInput {
        pub days: Option<Vec<Option<WeekDayInsertInput>>>,
        pub title: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "WeekDayUpdateInput")]
    pub struct WeekDayUpdateInput {
        pub meals: Option<Vec<Option<WeekDayMealUpdateInput>>>,
        pub title: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "WeekDayMealUpdateInput")]
    pub struct WeekDayMealUpdateInput {
        pub option: Option<String>,
        pub value: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "WeekDayInsertInput")]
    pub struct WeekDayInsertInput {
        pub meals: Option<Vec<Option<WeekDayMealInsertInput>>>,
        pub title: Option<String>,
    }

    #[derive(cynic::InputObject, Debug)]
    #[cynic(graphql_type = "WeekDayMealInsertInput")]
    pub struct WeekDayMealInsertInput {
        pub option: Option<String>,
        pub value: Option<String>,
    }

}

#[cynic::query_module(
    schema_path = r#"../schemas/schema.graphql"#,
    query_module = "query_dsl",
)]
mod types {
    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct ObjectId(pub String);

}

mod query_dsl{
    use super::types::*;
    cynic::query_dsl!(r#"../schemas/schema.graphql"#);
}

