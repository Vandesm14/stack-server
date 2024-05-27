export type Char = {
  char: string;
  index: number;
  line_index: number;
  line: number;
  wrapped: boolean;
};

export const max_lines = 7;
export const max_chars = 15;

export function stringToChars(string: string): Array<Char> {
  let chars: Array<Char> = [];
  let line = 0;
  let line_index = 0;
  let wrapping = false;

  string = string.replaceAll("\n", " \n");

  let new_line = () => {
    line_index = 0;
    line += 1;
  };

  let string_chars = string.split("");
  let count = 0;
  for (let i in string_chars) {
    let char = string_chars[i];
    let local_max = wrapping ? max_chars + 1 : max_chars;

    if (char === "\n") {
      new_line();
      wrapping = false;
      continue;
    }

    if (line_index >= local_max) {
      new_line();
      wrapping = true;
    }

    chars.push({
      char,
      index: count,
      line,
      line_index,
      wrapped: wrapping,
    });

    line_index += 1;
    count += 1;
  }

  return chars;
}
