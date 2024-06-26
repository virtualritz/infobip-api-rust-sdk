use crate::api::{
    sms::*,
    tests::{mock_blocking_json_endpoint, mock_json_endpoint, test_configuration, DUMMY_TEXT},
    SdkError,
};
use crate::model::sms::{ScheduledStatus::Paused, *};

const DUMMY_BASE_URL: &str = "https://some.url";

#[tokio::test]
async fn test_preview_valid() {
    let expected_response = r#"
       {
          "originalText": "Let's see how many characters remain unused in this message.",
          "previews": [
            {
              "textPreview": "Let's see how many characters remain unused in this message.",
              "messageCount": 1,
              "charactersRemaining": 96,
              "configuration": {}
            }
          ]
       }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_PREVIEW,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let request_body = PreviewRequestBody::new(DUMMY_TEXT);

    let response = client.preview(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.original_text.unwrap().is_empty());
    assert!(!response.body.previews.unwrap().is_empty());
}

#[tokio::test]
async fn test_preview_bad_request() {
    let client = SmsClient::with_configuration(test_configuration(DUMMY_BASE_URL));

    let mut request_body = PreviewRequestBody::new(DUMMY_TEXT);
    request_body.language_code = Some("XX".into());

    let error = client.preview(request_body).await.unwrap_err();

    if let SdkError::Validation(validation_error) = error {
        assert!(!validation_error.errors().is_empty());
    } else {
        panic!("not validation error")
    }
}

#[test]
fn test_blocking_preview_valid() {
    let expected_response = r#"
       {
          "originalText": "Let's see how many characters remain unused in this message.",
          "previews": [
            {
              "textPreview": "Let's see how many characters remain unused in this message.",
              "messageCount": 1,
              "charactersRemaining": 96,
              "configuration": {}
            }
          ]
       }
    "#;

    let mock_server = mock_blocking_json_endpoint(
        httpmock::Method::POST,
        PATH_PREVIEW,
        expected_response,
        reqwest::StatusCode::OK,
    );

    let client = BlockingSmsClient::with_configuration(test_configuration(&mock_server.base_url()));

    let request_body = PreviewRequestBody::new(DUMMY_TEXT);

    let response = client.preview(request_body).unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.original_text.unwrap().is_empty());
    assert!(!response.body.previews.unwrap().is_empty());
}

#[tokio::test]
async fn test_preview_server_error() {
    let expected_response = r#"
        {
          "requestError": {
            "serviceException": {
              "messageId": "string",
              "text": "string"
            }
          }
        }
    "#;
    let expected_status = reqwest::StatusCode::UNAUTHORIZED;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_PREVIEW,
        expected_response,
        expected_status,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let request_body = PreviewRequestBody::new(DUMMY_TEXT);

    let error = client.preview(request_body).await.unwrap_err();
    if let SdkError::ApiRequestError(api_error) = error {
        assert_eq!(api_error.status, expected_status);
        assert!(!api_error
            .details
            .request_error
            .service_exception
            .text
            .unwrap()
            .is_empty());
    } else {
        panic!("not an API error")
    }
}

#[tokio::test]
async fn test_delivery_reports_valid() {
    let expected_response = r#"
        {
          "results": [
            {
              "bulkId": "BULK-ID-123-xyz",
              "messageId": "MESSAGE-ID-123-xyz",
              "to": "41793026727",
              "sentAt": "2019-11-09T16:00:00.000+0000",
              "doneAt": "2019-11-09T16:00:00.000+0000",
              "smsCount": 1,
              "price": {
                "pricePerMessage": 0.01,
                "currency": "EUR"
              },
              "status": {
                "groupId": 3,
                "groupName": "DELIVERED",
                "id": 5,
                "name": "DELIVERED_TO_HANDSET",
                "description": "Message delivered to handset"
              },
              "error": {
                "groupId": 0,
                "groupName": "Ok",
                "id": 0,
                "name": "NO_ERROR",
                "description": "No Error",
                "permanent": false
              }
            },
            {
              "bulkId": "BULK-ID-123-xyz",
              "messageId": "12db39c3-7822-4e72-a3ec-c87442c0ffc5",
              "to": "41793026834",
              "sentAt": "2019-11-09T17:00:00.000+0000",
              "doneAt": "2019-11-09T17:00:00.000+0000",
              "smsCount": 1,
              "price": {
                "pricePerMessage": 0.01,
                "currency": "EUR"
              },
              "status": {
                "groupId": 3,
                "groupName": "DELIVERED",
                "id": 5,
                "name": "DELIVERED_TO_HANDSET",
                "description": "Message delivered to handset"
              },
              "error": {
                "groupId": 0,
                "groupName": "Ok",
                "id": 0,
                "name": "NO_ERROR",
                "description": "No Error",
                "permanent": false
              }
            }
          ]
        }
    "#;
    let expected_status = reqwest::StatusCode::OK;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_GET_DELIVERY_REPORTS,
        expected_response,
        expected_status,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let mut query_parameters = DeliveryReportsQueryParameters::new();
    query_parameters.limit = Some(10);

    let response = client.delivery_reports(query_parameters).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(response.body.results.as_ref().unwrap().len() > 1);
    assert!(!response.body.results.as_ref().unwrap()[0]
        .bulk_id
        .as_ref()
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn test_delivery_reports_bad_parameters() {
    let client = SmsClient::with_configuration(test_configuration(DUMMY_BASE_URL));

    let mut query_parameters = DeliveryReportsQueryParameters::new();
    query_parameters.limit = Some(10000);

    let error = client.delivery_reports(query_parameters).await.unwrap_err();
    if let SdkError::Validation(validation_error) = error {
        assert!(!validation_error.errors().is_empty());
    }
}

#[tokio::test]
async fn test_send_valid() {
    let expected_response = r#"
    {
      "bulkId": "2034072219640523073",
      "messages": [
        {
          "messageId": "41793026727",
          "status": {
            "description": "Message sent to next instance",
            "groupId": 1,
            "groupName": "PENDING",
            "id": 26,
            "name": "MESSAGE_ACCEPTED"
          },
          "to": "2033247207850523791"
        },
        {
          "messageId": "41793026834",
          "status": {
            "description": "Message sent to next instance",
            "groupId": 1,
            "groupName": "PENDING",
            "id": 26,
            "name": "MESSAGE_ACCEPTED"
          },
          "to": "2033247207850523792"
        }
      ]
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_SEND,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let message = Message::new(vec![Destination::new("123456789101")]);
    let request_body = SendRequestBody::new(vec![message]);

    let response = client.send(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.messages.unwrap().is_empty());
}

#[tokio::test]
async fn test_send_binary_valid() {
    let expected_response = r#"
    {
      "bulkId": "2034072219640523073",
      "messages": [
        {
          "messageId": "41793026727",
          "status": {
            "description": "Message sent to next instance",
            "groupId": 1,
            "groupName": "PENDING",
            "id": 26,
            "name": "MESSAGE_ACCEPTED"
          },
          "to": "2033247207850523791"
        },
        {
          "messageId": "41793026834",
          "status": {
            "description": "Message sent to next instance",
            "groupId": 1,
            "groupName": "PENDING",
            "id": 26,
            "name": "MESSAGE_ACCEPTED"
          },
          "to": "2033247207850523792"
        }
      ]
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_SEND_BINARY,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let message = BinaryMessage::new(vec![Destination::new("123456789101")]);
    let request_body = SendBinaryRequestBody::new(vec![message]);

    let response = client.send_binary(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.messages.unwrap().is_empty());
}

#[tokio::test]
async fn test_send_over_query_parameters_valid() {
    let expected_response = r#"
    {
      "bulkId": "1478260834465349756",
      "messages": [
        {
          "to": "41793026727",
          "status": {
            "groupId": 1,
            "groupName": "PENDING",
            "id": 26,
            "name": "PENDING_ACCEPTED",
            "description": "Message sent to next instance"
          },
          "messageId": "2250be2d4219-3af1-78856-aabe-1362af1edfd2"
        }
      ]
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_SEND_OVER_QUERY_PARAMS,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let query_parameters = SendOverQueryParametersQueryParameters::new(
        "username",
        "password",
        vec!["41793026727".to_string()],
    );

    let response = client
        .send_over_query_parameters(query_parameters)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.messages.unwrap().is_empty());
}

#[tokio::test]
async fn test_scheduled_valid() {
    let expected_response = r#"
        {
          "bulkId": "BULK-ID-123-xyz",
          "sendAt": "2021-08-25T16:00:00.000+0000"
        }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_GET_SCHEDULED,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let query_parameters = ScheduledQueryParameters::new("BULK-ID-123-xyz");

    let response = client.scheduled(query_parameters).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.bulk_id, "BULK-ID-123-xyz");
}

#[tokio::test]
async fn test_scheduled_empty_bulk_id() {
    let client = SmsClient::with_configuration(test_configuration("https://some.url"));

    let query_parameters = ScheduledQueryParameters::new("");

    assert!(client.scheduled(query_parameters).await.is_err());
}

#[tokio::test]
async fn test_reschedule_valid() {
    let expected_response = r#"
    {
      "bulkId": "BULK-ID-123-xyz",
      "sendAt": "2021-08-25T16:00:00.000+0000"
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::PUT,
        PATH_RESCHEDULE,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let query_parameters = RescheduleQueryParameters::new("BULK-ID-123-xyz");
    let request_body = RescheduleRequestBody::new("2021-08-25T16:00:00.000+0000");

    let response = client
        .reschedule(query_parameters, request_body)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.bulk_id, "BULK-ID-123-xyz");
    assert_eq!(response.body.send_at, "2021-08-25T16:00:00.000+0000");
}

#[tokio::test]
async fn test_reschedule_empty_bulk_id() {
    let client = SmsClient::with_configuration(test_configuration("https://some.url"));

    let query_parameters = RescheduleQueryParameters::new("");
    let request_body = RescheduleRequestBody::new("2021-08-25T16:00:00.000+0000");

    assert!(client
        .reschedule(query_parameters, request_body)
        .await
        .is_err());
}

#[tokio::test]
async fn test_reschedule_empty_send_at() {
    let client = SmsClient::with_configuration(test_configuration("https://some.url"));

    let query_parameters = RescheduleQueryParameters::new("BULK-ID-123-xyz");
    let request_body = RescheduleRequestBody::new("");

    assert!(client
        .reschedule(query_parameters, request_body)
        .await
        .is_err());
}

#[tokio::test]
async fn test_scheduled_status_valid() {
    let expected_response = r#"
    {
      "bulkId": "BULK-ID-123-xyz",
      "status": "PAUSED"
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_GET_SCHEDULED_STATUS,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let query_parameters = ScheduledStatusQueryParameters::new("BULK-ID-123-xyz");

    let response = client.scheduled_status(query_parameters).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.bulk_id.unwrap(), "BULK-ID-123-xyz");
    assert_eq!(response.body.status.unwrap(), Paused);
}

#[tokio::test]
async fn test_scheduled_status_empty_bulk_id() {
    let client = SmsClient::with_configuration(test_configuration("https://some.url"));

    let query_parameters = ScheduledStatusQueryParameters::new("");

    assert!(client.scheduled_status(query_parameters).await.is_err());
}

#[tokio::test]
async fn test_update_scheduled_status_valid() {
    let expected_response = r#"
    {
      "bulkId": "BULK-ID-123-xyz",
      "status": "PAUSED"
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::PUT,
        PATH_UPDATE_SCHEDULED_STATUS,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let query_parameters = UpdateScheduledStatusQueryParameters::new("BULK-ID-123-xyz");
    let request_body = UpdateScheduledStatusRequestBody::new(Paused);

    let response = client
        .update_scheduled_status(query_parameters, request_body)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.bulk_id.unwrap(), "BULK-ID-123-xyz");
    assert_eq!(response.body.status.unwrap(), Paused);
}

#[tokio::test]
async fn test_update_scheduled_status_empty_bulk_id() {
    let client = SmsClient::with_configuration(test_configuration("https://some.url"));

    let query_parameters = UpdateScheduledStatusQueryParameters::new("");
    let request_body = UpdateScheduledStatusRequestBody::new(Paused);

    assert!(client
        .update_scheduled_status(query_parameters, request_body)
        .await
        .is_err());
}

#[tokio::test]
async fn test_inbound_reports_valid() {
    let expected_response = r#"
    {
      "results": [
        {
          "messageId": "817790313235066447",
          "from": "385916242493",
          "to": "385921004026",
          "text": "QUIZ Correct answer is Paris",
          "cleanText": "Correct answer is Paris",
          "keyword": "QUIZ",
          "receivedAt": "2019-11-09T16:00:00.000+0000",
          "smsCount": 1,
          "price": {
            "pricePerMessage": 0,
            "currency": "EUR"
          },
          "callbackData": "callbackData"
        }
      ],
      "messageCount": 1,
      "pendingMessageCount": 0
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_GET_INBOUND,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let query_parameters = InboundReportsQueryParameters::new();

    let response = client.inbound_reports(query_parameters).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.message_count.unwrap(), 1);
}

#[tokio::test]
async fn test_inbound_reports_big_limit() {
    let client = SmsClient::with_configuration(test_configuration("https://some.url"));

    let mut query_parameters = InboundReportsQueryParameters::new();
    query_parameters.limit = Some(1001);

    assert!(client.inbound_reports(query_parameters).await.is_err());
}

#[tokio::test]
async fn test_logs_valid() {
    let expected_response = r#"
    {
      "results": [
        {
          "bulkId": "BULK-ID-123-xyz",
          "messageId": "MESSAGE-ID-123-xyz",
          "to": "41793026727",
          "sentAt": "2019-11-09T16:00:00.000+0000",
          "doneAt": "2019-11-09T16:00:00.000+0000",
          "smsCount": 1,
          "mccMnc": "22801",
          "price": {
            "pricePerMessage": 0.01,
            "currency": "EUR"
          },
          "status": {
            "groupId": 3,
            "groupName": "DELIVERED",
            "id": 5,
            "name": "DELIVERED_TO_HANDSET",
            "description": "Message delivered to handset"
          },
          "error": {
            "groupId": 0,
            "groupName": "Ok",
            "id": 0,
            "name": "NO_ERROR",
            "description": "No Error",
            "permanent": false
          }
        },
        {
          "bulkId": "BULK-ID-123-xyz",
          "messageId": "MESSAGE-ID-ijkl-45",
          "to": "41793026834",
          "sentAt": "2019-11-09T17:00:00.000+0000",
          "doneAt": "2019-11-09T17:00:00.000+0000",
          "smsCount": 1,
          "mccMnc": "22801",
          "price": {
            "pricePerMessage": 0.01,
            "currency": "EUR"
          },
          "status": {
            "groupId": 3,
            "groupName": "DELIVERED",
            "id": 5,
            "name": "DELIVERED_TO_HANDSET",
            "description": "Message delivered to handset"
          },
          "error": {
            "groupId": 0,
            "groupName": "Ok",
            "id": 0,
            "name": "NO_ERROR",
            "description": "No Error",
            "permanent": false
          }
        }
      ]
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_GET_LOGS,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let query_parameters = LogsQueryParameters::new();

    let response = client.logs(query_parameters).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.results.unwrap().len(), 2usize);
}

#[tokio::test]
async fn test_logs_big_limit() {
    let client = SmsClient::with_configuration(test_configuration("https://some.url"));

    let mut query_parameters = LogsQueryParameters::new();
    query_parameters.limit = Some(1001);

    assert!(client.logs(query_parameters).await.is_err());
}

#[tokio::test]
async fn test_tfa_applications_valid() {
    let expected_response = r#"
    [
        {
        "applicationId": "0933F3BC087D2A617AC6DCB2EF5B8A61",
        "name": "Test application BASIC 1",
        "configuration": {
        "pinAttempts": 10,
        "allowMultiplePinVerifications": true,
        "pinTimeToLive": "2h",
        "verifyPinLimit": "1/3s",
        "sendPinPerApplicationLimit": "10000/1d",
        "sendPinPerPhoneNumberLimit": "3/1d"
        },
        "enabled": true
        }
    ]
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_GET_TFA_APPLICATIONS,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let response = client.tfa_applications().await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.len(), 1usize);
}

#[tokio::test]
async fn test_create_tfa_application_valid() {
    let expected_response = r#"
    {
      "applicationId": "1234567",
      "name": "Application name",
      "configuration": {
        "pinAttempts": 5,
        "allowMultiplePinVerifications": true,
        "pinTimeToLive": "10m",
        "verifyPinLimit": "2/4s",
        "sendPinPerApplicationLimit": "5000/12h",
        "sendPinPerPhoneNumberLimit": "2/1d"
      },
      "enabled": true
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_CREATE_TFA_APPLICATION,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let request_body = CreateTfaApplicationRequestBody::new("Application name");

    let response = client.create_tfa_application(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.application_id.unwrap(), "1234567");
}

#[tokio::test]
async fn test_create_tfa_application_no_name() {
    let client = SmsClient::with_configuration(test_configuration("https://some.url"));

    let request_body = CreateTfaApplicationRequestBody::new("");

    assert!(client.create_tfa_application(request_body).await.is_err());
}

#[tokio::test]
async fn test_tfa_application_valid() {
    let expected_response = r#"
    {
      "applicationId": "1234567",
      "name": "Application name",
      "configuration": {
        "pinAttempts": 5,
        "allowMultiplePinVerifications": true,
        "pinTimeToLive": "10m",
        "verifyPinLimit": "2/4s",
        "sendPinPerApplicationLimit": "5000/12h",
        "sendPinPerPhoneNumberLimit": "2/1d"
      },
      "enabled": true
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        &PATH_GET_TFA_APPLICATION.replace("{appId}", "1234567"),
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let response = client.tfa_application("1234567").await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.application_id.unwrap(), "1234567");
}

#[tokio::test]
async fn test_update_tfa_application_valid() {
    let expected_response = r#"
    {
      "applicationId": "1234567",
      "name": "Application name 2",
      "configuration": {
        "pinAttempts": 5,
        "allowMultiplePinVerifications": true,
        "pinTimeToLive": "10m",
        "verifyPinLimit": "2/4s",
        "sendPinPerApplicationLimit": "5000/12h",
        "sendPinPerPhoneNumberLimit": "2/1d"
      },
      "enabled": true
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::PUT,
        &PATH_UPDATE_TFA_APPLICATION.replace("{appId}", "1234567"),
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let request_body = UpdateTfaApplicationRequestBody::new("Application name 2");

    let response = client
        .update_tfa_application("1234567", request_body)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.application_id.unwrap(), "1234567");
}

#[tokio::test]
async fn test_tfa_message_templates_valid() {
    let expected_response = r#"
    [
      {
        "messageId": "9C815F8AF3328",
        "applicationId": "HJ675435E3A6EA43432G5F37A635KJ8B",
        "pinPlaceholder": "{{pin}}",
        "messageText": "Your PIN is {{pin}}.",
        "pinLength": 4,
        "pinType": "NUMERIC",
        "language": "en",
        "repeatDTMF": "1#",
        "speechRate": 1
      }
    ]
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        &PATH_GET_TFA_MESSAGE_TEMPLATES.replace("{appId}", "HJ675435E3A6EA43432G5F37A635KJ8B"),
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let response = client
        .tfa_message_templates("HJ675435E3A6EA43432G5F37A635KJ8B")
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.len(), 1usize);
}

#[tokio::test]
async fn test_create_tfa_message_template_valid() {
    let expected_response = r#"
    {
      "pinPlaceholder": "{{pin}}",
      "messageText": "Your pin is {{pin}}",
      "pinLength": 4,
      "pinType": "ALPHANUMERIC",
      "language": "en",
      "senderId": "Infobip 2FA",
      "repeatDTMF": "1#",
      "speechRate": 1
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        &PATH_CREATE_TFA_MESSAGE_TEMPLATE.replace("{appId}", "HJ675435E3A6EA43432G5F37A635KJ8B"),
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let request_body =
        CreateTfaMessageTemplateRequestBody::new("Your pin is {{pin}}", PinType::Alphanumeric, 4);
    let application_id = "HJ675435E3A6EA43432G5F37A635KJ8B";

    let response = client
        .create_tfa_message_template(application_id, request_body)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.pin_length, 4);
}

#[tokio::test]
async fn test_create_tfa_message_template_empty_message_text() {
    let client = SmsClient::with_configuration(test_configuration("https://some.url"));

    let request_body = CreateTfaMessageTemplateRequestBody::new("", PinType::Alphanumeric, 4);

    assert!(client
        .create_tfa_message_template("some-app-id", request_body)
        .await
        .is_err());
}

#[tokio::test]
async fn test_tfa_message_template_valid() {
    let expected_response = r#"
    {
      "pinPlaceholder": "{{pin}}",
      "messageText": "Your pin is {{pin}}",
      "pinLength": 4,
      "pinType": "ALPHANUMERIC",
      "language": "en",
      "senderId": "Infobip 2FA",
      "repeatDTMF": "1#",
      "speechRate": 1
    }
    "#;

    let endpoint_path = &PATH_GET_TFA_MESSAGE_TEMPLATE
        .replace("{appId}", "HJ675435E3A6EA43432G5F37A635KJ8B")
        .replace("{msgId}", "16A8B5FE2BCD6CA716A2D780CB3F3390");
    let server = mock_json_endpoint(
        httpmock::Method::GET,
        endpoint_path,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let response = client
        .tfa_message_template(
            "HJ675435E3A6EA43432G5F37A635KJ8B",
            "16A8B5FE2BCD6CA716A2D780CB3F3390",
        )
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.pin_length, 4);
}

#[tokio::test]
async fn test_update_tfa_message_template_valid() {
    let expected_response = r#"
    {
      "pinPlaceholder": "{{pin}}",
      "messageText": "Your pin is {{pin}}",
      "pinLength": 6,
      "pinType": "ALPHANUMERIC",
      "language": "en",
      "senderId": "Infobip 2FA",
      "repeatDTMF": "1#",
      "speechRate": 1
    }
    "#;

    let endpoint_path = &PATH_UPDATE_TFA_MESSAGE_TEMPLATE
        .replace("{appId}", "HJ675435E3A6EA43432G5F37A635KJ8B")
        .replace("{msgId}", "16A8B5FE2BCD6CA716A2D780CB3F3390");
    let server = mock_json_endpoint(
        httpmock::Method::PUT,
        endpoint_path,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let request_body =
        UpdateTfaMessageTemplateRequestBody::new("Your pin is {{pin}}", PinType::Alphanumeric, 6);

    let response = client
        .update_tfa_message_template(
            "HJ675435E3A6EA43432G5F37A635KJ8B",
            "16A8B5FE2BCD6CA716A2D780CB3F3390",
            request_body,
        )
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.pin_length, 6);
}

#[tokio::test]
async fn test_send_pin_over_sms_valid() {
    let expected_response = r#"
    {
      "pinId": "9C817C6F8AF3D48F9FE553282AFA2B67",
      "to": "41793026727",
      "ncStatus": "NC_DESTINATION_REACHABLE",
      "smsStatus": "MESSAGE_SENT"
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_SEND_PIN_OVER_SMS,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let query_parameters = SendPinOverSmsQueryParameters::default();
    let request_body = SendPinOverSmsRequestBody::new(
        "HJ675435E3A6EA43432G5F37A635KJ8B",
        "16A8B5FE2BCD6CA716A2D780CB3F3390",
        "5555555555",
    );

    let response = client
        .send_pin_over_sms(query_parameters, request_body)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(
        response.body.pin_id.unwrap(),
        "9C817C6F8AF3D48F9FE553282AFA2B67"
    );
}

#[tokio::test]
async fn test_send_pin_over_sms_empty_app_id() {
    let client = SmsClient::with_configuration(test_configuration("https://some.url"));

    let query_parameters = SendPinOverSmsQueryParameters::default();
    let request_body =
        SendPinOverSmsRequestBody::new("", "16A8B5FE2BCD6CA716A2D780CB3F3390", "5555555555");

    assert!(client
        .send_pin_over_sms(query_parameters, request_body)
        .await
        .is_err());
}

#[tokio::test]
async fn test_resend_pin_over_sms_valid() {
    let expected_response = r#"
    {
      "pinId": "9C817C6F8AF3D48F9FE553282AFA2B67",
      "to": "41793026727",
      "ncStatus": "NC_DESTINATION_REACHABLE",
      "smsStatus": "MESSAGE_SENT"
    }
    "#;

    let endpoint_path =
        &PATH_RESEND_PIN_OVER_SMS.replace("{pinId}", "9C817C6F8AF3D48F9FE553282AFA2B67");

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        endpoint_path,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let request_body = ResendPinOverSmsRequestBody::new();

    let response = client
        .resend_pin_over_sms("9C817C6F8AF3D48F9FE553282AFA2B67", request_body)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(
        response.body.pin_id.unwrap(),
        "9C817C6F8AF3D48F9FE553282AFA2B67"
    );
}

#[tokio::test]
async fn test_send_pin_over_voice_valid() {
    let expected_response = r#"
    {
      "pinId": "9C817C6F8AF3D48F9FE553282AFA2B67",
      "to": "41793026727",
      "callStatus": "PENDING_ACCEPTED"
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_SEND_PIN_OVER_VOICE,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let request_body = SendPinOverVoiceRequestBody::new(
        "HJ675435E3A6EA43432G5F37A635KJ8B",
        "16A8B5FE2BCD6CA716A2D780CB3F3390",
        "5555555555",
    );

    let response = client.send_pin_over_voice(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(
        response.body.pin_id.unwrap(),
        "9C817C6F8AF3D48F9FE553282AFA2B67"
    );
}

#[tokio::test]
async fn test_send_pin_over_voice_empty_app_id() {
    let client = SmsClient::with_configuration(test_configuration("https://some.url"));

    let request_body =
        SendPinOverVoiceRequestBody::new("", "16A8B5FE2BCD6CA716A2D780CB3F3390", "5555555555");

    assert!(client.send_pin_over_voice(request_body).await.is_err());
}

#[tokio::test]
async fn test_resend_pin_over_voice_valid() {
    let expected_response = r#"
    {
      "pinId": "9C817C6F8AF3D48F9FE553282AFA2B67",
      "to": "41793026727",
      "callStatus": "PENDING_ACCEPTED"
    }
    "#;

    let endpoint_path =
        &PATH_RESEND_PIN_OVER_VOICE.replace("{pinId}", "9C817C6F8AF3D48F9FE553282AFA2B67");

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        endpoint_path,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let request_body = ResendPinOverVoiceRequestBody::new();

    let response = client
        .resend_pin_over_voice("9C817C6F8AF3D48F9FE553282AFA2B67", request_body)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(
        response.body.pin_id.unwrap(),
        "9C817C6F8AF3D48F9FE553282AFA2B67"
    );
}

#[tokio::test]
async fn test_verify_phone_number_valid() {
    let expected_response = r#"
    {
      "pinId": "9C817C6F8AF3D48F9FE553282AFA2B67",
      "msisdn": "41793026727",
      "verified": true,
      "attemptsRemaining": 0
    }
    "#;

    let endpoint_path =
        &PATH_VERIFY_PHONE_NUMBER.replace("{pinId}", "9C817C6F8AF3D48F9FE553282AFA2B67");

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        endpoint_path,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let request_body = VerifyPhoneNumberRequestBody::new("123456");

    let response = client
        .verify_phone_number("9C817C6F8AF3D48F9FE553282AFA2B67", request_body)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(
        response.body.pin_id.unwrap(),
        "9C817C6F8AF3D48F9FE553282AFA2B67"
    );
}

#[tokio::test]
async fn test_verify_phone_number_no_pin() {
    let client = SmsClient::with_configuration(test_configuration("https://some.url"));

    let request_body = VerifyPhoneNumberRequestBody::new("");

    assert!(client
        .verify_phone_number("some-pin-id", request_body)
        .await
        .is_err());
}

#[tokio::test]
async fn test_tfa_verification_status_valid() {
    let expected_response = r#"
    {
      "verifications": [
        {
          "msisdn": "41793026727",
          "verified": true,
          "verifiedAt": 1418364366,
          "sentAt": 1418364246
        }
      ]
    }
    "#;

    let endpoint_path =
        &PATH_GET_TFA_VERIFICATION_STATUS.replace("{appId}", "16A8B5FE2BCD6CA716A2D780CB3F3390");

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        endpoint_path,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(test_configuration(&server.base_url()));

    let query_parameters = TfaVerificationStatusQueryParameters::new("41793026727");

    let response = client
        .tfa_verification_status("16A8B5FE2BCD6CA716A2D780CB3F3390", query_parameters)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.verifications.unwrap().len(), 1usize);
}

#[tokio::test]
async fn test_tfa_verification_status_empty_msisdn() {
    let client = SmsClient::with_configuration(test_configuration("https://some.url"));

    let query_parameters = TfaVerificationStatusQueryParameters::new("");

    assert!(client
        .tfa_verification_status("some-app-id", query_parameters)
        .await
        .is_err());
}
