/// After looking at the solution I realized I forgot to include the "escalate" ability.
/// I might revisit this in the future and also add a "call_queue", but to be honest I the OOP
/// section of the book is not really what I was looking to practice.
///
/// Additionally OOP interviews don't ususally require full implementations, but rather a sketch of
/// the overall architecture, so this is of low priority for me
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

    pub fn dispatch_call(&mut self, role: Role) -> Result<Call, String> {
        if let (false, true) = (self.respondents.is_empty(), role == Role::Respondent) {
            return Ok(Call::RespondentCall(self.respondents.pop_front().unwrap()));
        }
        if let (false, true) = (self.managers.is_empty(), role <= Role::Manager) {
            return Ok(Call::ManagerCall(self.managers.pop_front().unwrap()));
        }
        if let (false, true) = (self.directors.is_empty(), role <= Role::Director) {
            return Ok(Call::DirectorCall(self.directors.pop_front().unwrap()));
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Role {
    Respondent = 0,
    Manager = 1,
    Director = 2,
}

impl Default for Role {
    fn default() -> Self {
        Self::Respondent
    }
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

        let first_call = call_center.dispatch_call(Default::default()).unwrap();
        assert_eq!(first_call.name(), "James");
        assert_eq!(first_call.role(), Role::Respondent);

        let second_call = call_center.dispatch_call(Default::default()).unwrap();
        assert_eq!(second_call.name(), "George");
        assert_eq!(second_call.role(), Role::Manager);

        let third_call = call_center.dispatch_call(Default::default()).unwrap();
        assert_eq!(third_call.name(), "Patrick");
        assert_eq!(third_call.role(), Role::Director);
    }

    #[test]
    fn call_center_2() {
        let mut call_center = CallCenter::from_iters(["James"], ["George"], ["Patrick"]);

        let first_call = call_center.dispatch_call(Role::Manager).unwrap();
        assert_eq!(first_call.name(), "George");
        assert_eq!(first_call.role(), Role::Manager);

        let second_call = call_center.dispatch_call(Default::default()).unwrap();
        assert_eq!(second_call.name(), "James");
        assert_eq!(second_call.role(), Role::Respondent);

        call_center.end_call(first_call);

        let third_call = call_center.dispatch_call(Default::default()).unwrap();
        assert_eq!(third_call.name(), "George");
        assert_eq!(third_call.role(), Role::Manager);

        let fourth_call = call_center.dispatch_call(Default::default()).unwrap();
        assert_eq!(fourth_call.name(), "Patrick");
        assert_eq!(fourth_call.role(), Role::Director);
    }
}
