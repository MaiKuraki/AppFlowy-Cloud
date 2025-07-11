use std::collections::HashSet;

use client_api::entity::{AFRole, WorkspaceMemberProfile};
use client_api_test::TestClient;

#[tokio::test]
async fn workspace_mentionable_persons_crud() {
  let owner = TestClient::new_user().await;
  let guest = TestClient::new_user().await;
  let guest_name = guest.api_client.get_profile().await.unwrap().name.unwrap();
  let workspace_id = owner.workspace_id().await;
  owner
    .invite_and_accepted_workspace_member(&workspace_id, &guest, AFRole::Guest)
    .await
    .unwrap();
  let workspaces = owner.api_client.get_workspaces().await.unwrap();
  assert_eq!(workspaces.len(), 1);
  let workspace_id = workspaces[0].workspace_id;
  owner
    .api_client
    .update_workspace_member_profile(
      &workspace_id,
      &WorkspaceMemberProfile {
        name: "name override".to_string(),
        avatar_url: Some("avatar url override".to_string()),
        cover_image_url: Some("cover image url".to_string()),
        description: Some("description override".to_string()),
      },
    )
    .await
    .unwrap();

  let mentionable_persons = owner
    .api_client
    .list_workspace_mentionable_persons(&workspace_id)
    .await
    .unwrap()
    .persons;
  assert_eq!(mentionable_persons.len(), 2);
  let mentionable_person_names: HashSet<String> =
    mentionable_persons.iter().map(|p| p.name.clone()).collect();
  assert!(mentionable_person_names.contains("name override"));
  assert!(mentionable_person_names.contains(&guest_name));
  let person_id = mentionable_persons
    .iter()
    .find(|p| p.name == guest_name)
    .unwrap()
    .uuid;
  let mentionable_person = owner
    .api_client
    .get_workspace_mentionable_person(&workspace_id, &person_id)
    .await
    .unwrap();
  assert_eq!(mentionable_person.name, guest_name);
}
