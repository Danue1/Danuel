pub struct Quest {
    title: String,
    state: QuestState,
}

impl Quest {
    pub fn new(title: String, state: QuestState) -> Self {
        Quest { title, state }
    }

    pub fn lock(&mut self) {
        self.state.lock();
    }

    pub fn unlock(&mut self) {
        self.state.unlock();
    }

    pub fn pending(&mut self) {
        self.state.pending();
    }
}

impl Quest {
    pub fn title(&self) -> String {
        self.title.to_owned()
    }

    pub fn state(&self) -> QuestState {
        self.state
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum QuestState {
    Locked,
    Unlocked,
    Pending,
    InProgress,
    Completed,
    Done,
    Canceled,
}

impl QuestState {
    pub fn lock(&mut self) {
        debug_assert_ne!(*self, QuestState::Locked);
        debug_assert_ne!(*self, QuestState::Done);
        debug_assert_ne!(*self, QuestState::Completed);
        debug_assert_ne!(*self, QuestState::InProgress);
        *self = QuestState::Locked;
    }

    pub fn unlock(&mut self) {
        debug_assert_eq!(*self, QuestState::Locked);
        debug_assert_ne!(*self, QuestState::Unlocked);
        *self = QuestState::Unlocked;
    }

    pub fn pending(&mut self) {
        debug_assert_eq!(*self, QuestState::Unlocked);
        debug_assert_ne!(*self, QuestState::Pending);
        *self = QuestState::Pending;
    }
}
