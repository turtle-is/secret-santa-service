use std::collections::HashMap;
use std::sync::{Mutex, Arc};


#[derive(Clone, Debug)]
struct Group {
    name: String,
    creator: String,
    members: Vec<String>,
    admins: Vec<String>,
    closed: bool,
    secret_santas: HashMap<String, String>,
}

#[derive(Clone, Debug)]
struct User {
    name: String,
    groups: Vec<String>,
    secret_santas: HashMap<String, String>,
}

#[derive(Clone, Debug)]
struct Service {
    users: Mutex<HashMap<String, User>>,
    groups: Mutex<HashMap<String, Group>>,
}

impl Service {
    fn new() -> Self {
        Service {
            users: Mutex::new(HashMap::new()),
            groups: Mutex::new(HashMap::new()),
        }
    }

    fn create_group(&self, group_name: String, creator: String) -> Result<(), String> {
        let mut groups = self.groups.lock().unwrap();
        if groups.contains_key(&group_name) {
            return Err(format!("Group {} already exists", group_name));
        }
        let mut users = self.users.lock().unwrap();
        if !users.contains_key(&creator) {
            return Err(format!("User {} does not exist", creator));
        }

        let group = Group {
            name: group_name,
            creator,
            members: vec![creator.clone()],
            admins: vec![creator.clone()],
            closed: false,
            secret_santas: HashMap::new(),
        };
        groups.insert(group_name, group);

        let user = users.get_mut(&creator).unwrap();
        user.groups.push(group_name.clone());
        Ok(())
    }
   }

    fn join_group(&self, group_name: String, user: String) -> Result<(), String> {
        let mut groups = self.groups.lock().unwrap();
        let group = groups.get_mut(&group_name).ok_or(format!("Group {} does not exist", group_name))?;
        if group.closed {
            return Err(format!("Group {} is closed", group_name));
        }
        let mut users = self.users.lock().unwrap();
        if !users.contains_key(&user) {
            return Err(format!("User {} does not exist", user));
        }
        if group.members.contains(&user) {
            return Err(format!("User {} is already a member of group {}", user, group_name));
        }
        group.members.push(user.clone());
      }