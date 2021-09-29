use std::collections::VecDeque;

pub struct CallCenter {
    respondents: VecDeque<Respondent>,
    managers: VecDeque<Manager>,
    directors: VecDeque<Director>,
}

pub enum Call {
    RespondentCall(Respondent),
    ManagerCall(Manager),
    DirectorCall(Director),
}

/// Added a couple helper methods to help with testing
impl Call {
    pub fn name(&self) -> &str {
        match self {
            Call::RespondentCall(employee) => employee.name(),
            Call::ManagerCall(employee) => employee.name(),
            Call::DirectorCall(employee) => employee.name(),
        }
    }

    pub fn role(&self) -> Role {
        match self {
            Call::RespondentCall(employee) => employee.role(),
            Call::ManagerCall(employee) => employee.role(),
            Call::DirectorCall(employee) => employee.role(),
        }
    }
}

impl CallCenter {
    pub fn new() -> Self {
        Self {
            respondents: VecDeque::new(),
            managers: VecDeque::new(),
            directors: VecDeque::new(),
        }
    }

    pub fn from_iters<'a, I: IntoIterator<Item = &'a str>>(
        respondents: I,
        managers: I,
        directors: I,
    ) -> Self {
        Self {
            respondents: Respondent::names_to_queue(respondents),
            managers: Manager::names_to_queue(managers),
            directors: Director::names_to_queue(directors),
        }
    }

    pub fn dispatch_call(&mut self) -> Result<Call, String> {
        if let Some(respondent) = self.respondents.pop_front() {
            return Ok(Call::RespondentCall(respondent));
        }
        if let Some(manager) = self.managers.pop_front() {
            return Ok(Call::ManagerCall(manager));
        }
        if let Some(director) = self.directors.pop_front() {
            return Ok(Call::DirectorCall(director));
        }
        Err(String::from("No one is available"))
    }

    pub fn end_call(&mut self, call: Call) {
        match call {
            Call::RespondentCall(respondent) => self.respondents.push_back(respondent),
            Call::ManagerCall(manager) => self.managers.push_back(manager),
            Call::DirectorCall(director) => self.directors.push_back(director),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Respondent,
    Manager,
    Director,
}

pub struct Respondent {
    name: String,
    role: Role,
}

impl Respondent {
    fn name(&self) -> &str {
        &self.name
    }

    fn role(&self) -> Role {
        self.role
    }
}

impl Respondent {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
            role: Role::Respondent,
        }
    }

    fn names_to_queue<'a, I: IntoIterator<Item = &'a str>>(names: I) -> VecDeque<Self> {
        names.into_iter().map(|n| Self::from(n)).collect()
    }
}

pub struct Manager {
    name: String,
    role: Role,
}

impl Manager {
    fn name(&self) -> &str {
        &self.name
    }

    fn role(&self) -> Role {
        self.role
    }
}

impl Manager {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
            role: Role::Manager,
        }
    }

    fn names_to_queue<'a, I: IntoIterator<Item = &'a str>>(names: I) -> VecDeque<Self> {
        names.into_iter().map(|n| Self::from(n)).collect()
    }
}

pub struct Director {
    name: String,
    role: Role,
}

impl Director {
    fn name(&self) -> &str {
        &self.name
    }

    fn role(&self) -> Role {
        self.role
    }
}

impl Director {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
            role: Role::Director,
        }
    }

    fn names_to_queue<'a, I: IntoIterator<Item = &'a str>>(names: I) -> VecDeque<Self> {
        names.into_iter().map(|n| Self::from(n)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_center_1() {
        let mut call_center = CallCenter::from_iters(["James"], ["George"], ["Patrick"]);

        let first_call = call_center.dispatch_call().unwrap();
        assert_eq!(first_call.name(), "James");
        assert_eq!(first_call.role(), Role::Respondent);

        let second_call = call_center.dispatch_call().unwrap();
        assert_eq!(second_call.name(), "George");
        assert_eq!(second_call.role(), Role::Manager);

        let third_call = call_center.dispatch_call().unwrap();
        assert_eq!(third_call.name(), "Patrick");
        assert_eq!(third_call.role(), Role::Director);
    }

    #[test]
    fn call_center_2() {
        let mut call_center = CallCenter::from_iters(["James"], ["George"], ["Patrick"]);

        let first_call = call_center.dispatch_call().unwrap();
        assert_eq!(first_call.name(), "James");
        assert_eq!(first_call.role(), Role::Respondent);

        let second_call = call_center.dispatch_call().unwrap();
        assert_eq!(second_call.name(), "George");
        assert_eq!(second_call.role(), Role::Manager);

        call_center.end_call(first_call);

        let third_call = call_center.dispatch_call().unwrap();
        assert_eq!(third_call.name(), "James");
        assert_eq!(third_call.role(), Role::Respondent);

        let fourth_call = call_center.dispatch_call().unwrap();
        assert_eq!(fourth_call.name(), "Patrick");
        assert_eq!(fourth_call.role(), Role::Director);
    }
}
