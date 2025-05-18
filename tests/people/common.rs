use lumx_sea_orm::sea_orm::{ActiveModelTrait, ActiveValue, DbErr};
use lumx_sea_orm::sea_orm::{DatabaseConnection, EntityTrait};
use portal_schema::{person, person_document_type, person_gender};

pub const PEOPLE_URL: &str = "/api/v1/people";
pub const PERSON_GENDERS_URL: &str = "/api/v1/person-genders";

pub async fn insert_person_gender_female(
    conn: &DatabaseConnection,
) -> Result<person_gender::ActiveModel, DbErr> {
    let gender_model = person_gender::ActiveModel {
        name: ActiveValue::Set("Female".to_string()),
        summary: ActiveValue::Set(Some("Female".to_string())),
        signature: ActiveValue::Set("273b9ae535de53399c86a9b83148a8ed".to_string()),
        ..Default::default()
    };
    gender_model.save(conn).await
}

pub async fn insert_person_gender_male(
    conn: &DatabaseConnection,
) -> Result<person_gender::ActiveModel, DbErr> {
    let gender_model = person_gender::ActiveModel {
        name: ActiveValue::Set("Male".to_string()),
        summary: ActiveValue::Set(Some("Male".to_string())),
        signature: ActiveValue::Set("".to_string()),
        ..Default::default()
    };
    gender_model.save(conn).await
}

pub async fn insert_document_dni(
    conn: &DatabaseConnection,
) -> Result<person_document_type::ActiveModel, DbErr> {
    let document_type_model = person_document_type::ActiveModel {
        name: ActiveValue::Set("DNI".to_string()),
        summary: ActiveValue::Set(Some("DNI".to_string())),
        signature: ActiveValue::Set("".to_string()),
        ..Default::default()
    };
    document_type_model.save(conn).await
}

pub async fn insert_document_piva(
    conn: &DatabaseConnection,
) -> Result<person_document_type::ActiveModel, DbErr> {
    let document_type_model = person_document_type::ActiveModel {
        name: ActiveValue::Set("P.IVA".to_string()),
        summary: ActiveValue::Set(Some("Partiva IVA".to_string())),
        signature: ActiveValue::Set("".to_string()),
        ..Default::default()
    };
    document_type_model.save(conn).await
}

pub async fn insert_person_sample_1(conn: &DatabaseConnection) {
    let person_gender = insert_person_gender_female(conn).await.unwrap();
    let document_type = insert_document_piva(conn).await.unwrap();

    let person_model = person::ActiveModel {
        first_name: ActiveValue::Set("Idesoft".to_string()),
        last_name: ActiveValue::Set("Systems".to_string()),
        document_number: ActiveValue::Set("ID3SOFT".to_string()),
        document_type_id: ActiveValue::Set(document_type.person_document_type_id.unwrap()),
        gender_id: ActiveValue::Set(person_gender.person_gender_id.unwrap()),
        signature: ActiveValue::Set("87ca18b2b4e8fa897cc0bfdf04c58417".to_string()),
        ..Default::default()
    };
    person_model.save(conn).await.unwrap();
}

pub async fn insert_person_sample_2(conn: &DatabaseConnection) {
    let person_gender = insert_person_gender_male(conn).await.unwrap();
    let document_type = insert_document_dni(conn).await.unwrap();

    let person_model = person::ActiveModel {
        first_name: ActiveValue::Set("Person".to_string()),
        last_name: ActiveValue::Set("One".to_string()),
        document_number: ActiveValue::Set("0001".to_string()),
        document_type_id: ActiveValue::Set(document_type.person_document_type_id.unwrap()),
        gender_id: ActiveValue::Set(person_gender.person_gender_id.unwrap()),
        signature: ActiveValue::Set("a0babc75f468dc49b6382b60653eadaa".to_string()),
        ..Default::default()
    };
    person_model.save(conn).await.unwrap();
}

pub async fn insert_people_sample_to_paginate(conn: &DatabaseConnection) {
    insert_person_gender_female(conn).await.unwrap();
    insert_person_gender_male(conn).await.unwrap();

    insert_document_piva(conn).await.unwrap();

    let person_model_1 = person::ActiveModel {
        first_name: ActiveValue::Set("Person".to_string()),
        last_name: ActiveValue::Set("One".to_string()),
        document_number: ActiveValue::Set("0001".to_string()),
        document_type_id: ActiveValue::Set(1),
        gender_id: ActiveValue::set(1),
        signature: ActiveValue::Set("78138cbcfb203ae9af9872bbd7364143".to_string()),
        ..Default::default()
    };
    let person_model_2 = person::ActiveModel {
        first_name: ActiveValue::Set("Person".to_string()),
        last_name: ActiveValue::Set("Two".to_string()),
        document_number: ActiveValue::Set("0002".to_string()),
        document_type_id: ActiveValue::Set(1),
        gender_id: ActiveValue::set(2),
        signature: ActiveValue::Set("78138cbcfb203ae9af9872bbd7364143".to_string()),
        ..Default::default()
    };
    let person_model_3 = person::ActiveModel {
        first_name: ActiveValue::Set("Person".to_string()),
        last_name: ActiveValue::Set("Three".to_string()),
        document_number: ActiveValue::Set("0003".to_string()),
        document_type_id: ActiveValue::Set(1),
        gender_id: ActiveValue::set(1),
        signature: ActiveValue::Set("78138cbcfb203ae9af9872bbd7364143".to_string()),
        ..Default::default()
    };
    let person_model_4 = person::ActiveModel {
        first_name: ActiveValue::Set("Person".to_string()),
        last_name: ActiveValue::Set("Four".to_string()),
        document_number: ActiveValue::Set("0004".to_string()),
        document_type_id: ActiveValue::Set(1),
        gender_id: ActiveValue::set(1),
        signature: ActiveValue::Set("78138cbcfb203ae9af9872bbd7364143".to_string()),
        ..Default::default()
    };
    let person_model_5 = person::ActiveModel {
        first_name: ActiveValue::Set("Person".to_string()),
        last_name: ActiveValue::Set("Five".to_string()),
        document_number: ActiveValue::Set("0005".to_string()),
        document_type_id: ActiveValue::Set(1),
        gender_id: ActiveValue::set(1),
        signature: ActiveValue::Set("78138cbcfb203ae9af9872bbd7364143".to_string()),
        ..Default::default()
    };
    let person_model_6 = person::ActiveModel {
        first_name: ActiveValue::Set("Person".to_string()),
        last_name: ActiveValue::Set("Six".to_string()),
        document_number: ActiveValue::Set("0006".to_string()),
        document_type_id: ActiveValue::Set(1),
        gender_id: ActiveValue::set(1),
        signature: ActiveValue::Set("78138cbcfb203ae9af9872bbd7364143".to_string()),
        ..Default::default()
    };
    let person_model_7 = person::ActiveModel {
        first_name: ActiveValue::Set("Person".to_string()),
        last_name: ActiveValue::Set("Seven".to_string()),
        document_number: ActiveValue::Set("0007".to_string()),
        document_type_id: ActiveValue::Set(1),
        gender_id: ActiveValue::set(1),
        signature: ActiveValue::Set("78138cbcfb203ae9af9872bbd7364143".to_string()),
        ..Default::default()
    };
    let person_model_8 = person::ActiveModel {
        first_name: ActiveValue::Set("Person".to_string()),
        last_name: ActiveValue::Set("Eight".to_string()),
        document_number: ActiveValue::Set("0008".to_string()),
        document_type_id: ActiveValue::Set(1),
        gender_id: ActiveValue::set(1),
        signature: ActiveValue::Set("78138cbcfb203ae9af9872bbd7364143".to_string()),
        ..Default::default()
    };
    let person_model_9 = person::ActiveModel {
        first_name: ActiveValue::Set("Person".to_string()),
        last_name: ActiveValue::Set("Nine".to_string()),
        document_number: ActiveValue::Set("0009".to_string()),
        document_type_id: ActiveValue::Set(1),
        gender_id: ActiveValue::set(1),
        signature: ActiveValue::Set("78138cbcfb203ae9af9872bbd7364143".to_string()),
        ..Default::default()
    };
    let person_model_10 = person::ActiveModel {
        first_name: ActiveValue::Set("Person".to_string()),
        last_name: ActiveValue::Set("Ten".to_string()),
        document_number: ActiveValue::Set("0010".to_string()),
        document_type_id: ActiveValue::Set(1),
        gender_id: ActiveValue::set(1),
        signature: ActiveValue::Set("78138cbcfb203ae9af9872bbd7364143".to_string()),
        ..Default::default()
    };
    let person_model_11 = person::ActiveModel {
        first_name: ActiveValue::Set("Person".to_string()),
        last_name: ActiveValue::Set("Eleven".to_string()),
        document_number: ActiveValue::Set("0011".to_string()),
        document_type_id: ActiveValue::Set(1),
        gender_id: ActiveValue::set(1),
        signature: ActiveValue::Set("78138cbcfb203ae9af9872bbd7364143".to_string()),
        ..Default::default()
    };
    let person_model_12 = person::ActiveModel {
        first_name: ActiveValue::Set("Person".to_string()),
        last_name: ActiveValue::Set("Twelve".to_string()),
        document_number: ActiveValue::Set("0012".to_string()),
        document_type_id: ActiveValue::Set(1),
        gender_id: ActiveValue::set(1),
        signature: ActiveValue::Set("78138cbcfb203ae9af9872bbd7364143".to_string()),
        ..Default::default()
    };
    let person_model_13 = person::ActiveModel {
        first_name: ActiveValue::Set("Person".to_string()),
        last_name: ActiveValue::Set("Thirteen".to_string()),
        document_number: ActiveValue::Set("0013".to_string()),
        document_type_id: ActiveValue::Set(1),
        gender_id: ActiveValue::set(1),
        signature: ActiveValue::Set("78138cbcfb203ae9af9872bbd7364143".to_string()),
        ..Default::default()
    };
    person::Entity::insert_many([
        person_model_1,
        person_model_2,
        person_model_3,
        person_model_4,
        person_model_5,
        person_model_6,
        person_model_7,
        person_model_8,
        person_model_9,
        person_model_10,
        person_model_11,
        person_model_12,
        person_model_13,
    ])
    .exec(conn)
    .await
    .unwrap();
}
