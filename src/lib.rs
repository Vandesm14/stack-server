#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Character {
  pub char: char,
  pub index: usize,
  pub line_index: usize,
  pub line: usize,
  pub wrapped: bool,
}

pub fn string_to_chars(string: &str, max_chars: usize) -> Vec<Character> {
  let mut chars: Vec<Character> = Vec::new();
  let mut line = 0;
  let mut line_index = 0;
  let mut wrapping = false;

  let mut count = 0;
  for char in string.chars() {
    let local_max = if wrapping { max_chars + 1 } else { max_chars };

    if char == '\n' {
      line_index = 0;
      line += 1;
      wrapping = false;
      continue;
    }

    if line_index >= local_max {
      line_index = 0;
      line += 1;
      wrapping = true;
    }

    chars.push(Character {
      char,
      index: count,
      line,
      line_index,
      wrapped: wrapping,
    });

    line_index += 1;
    count += 1;
  }

  chars
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum EditorMode {
  #[default]
  Edit,
  Run,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum MoveAction {
  #[default]
  Invalid,

  // Modal
  Mode,

  // Navigation
  Home,
  End,
  Left,
  Right,
  Up,
  Down,

  // Input
  Enter,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Editor {
  pub cursor: usize,
  pub line_offset: usize,

  pub code: String,
  pub chars: Vec<Character>,
  pub buffer: String,

  pub mode: EditorMode,
}

impl Editor {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub fn with_code(mut self, code: String) -> Self {
    self.code = code;
    self.refresh_chars();
    self
  }

  pub fn set_cursor(&mut self, cursor: usize) {
    self.cursor = cursor.clamp(0, self.buffer.len() - 1);
  }

  pub fn navigate(&mut self, action: MoveAction) {
    match action {
      MoveAction::Left => self.set_cursor(self.cursor - 1),
      MoveAction::Right => self.set_cursor(self.cursor + 1),

      _ => {}
    }
  }

  pub fn refresh_chars(&mut self) {
    self.buffer = self.code.replace('\n', " \n");

    if !self.buffer.ends_with(' ') {
      self.buffer.push(' ');
    }

    self.chars = string_to_chars(&self.buffer, 15);
  }
}
