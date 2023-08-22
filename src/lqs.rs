/// Clauses used by LQS format
#[derive(PartialEq, Clone, Copy)]
enum Clause {
    Select,
    From,
    Where,
    Skip,
}

impl Clause {
    fn as_str(&self) -> &'static str {
        match self {
            Clause::Select => "select",
            Clause::From => "from",
            Clause::Where => "where",
            Clause::Skip => "skip",
        }
    }
}

/// State of query parser
struct ParserState {
    current_clause: Clause,
    last_clause: Clause,
}

impl ParserState {
    pub fn new() -> Self {
        Self {
            current_clause: Clause::From,
            last_clause: Clause::From,
        }
    }

    pub fn set(&mut self, new_state: Clause) {
        self.last_clause = self.current_clause.clone();
        self.current_clause = new_state.clone();
    }
}

/// LQS query parser
pub fn parse(q: String) -> String {
    let mut lower_q = q.to_lowercase();

    // Remove end semicolon
    if lower_q.ends_with(";") {
        let mut chars = lower_q.chars();
        chars.next_back();
        lower_q = chars.as_str().to_string();
    }

    // tokenize string and parse each word at a time with logic and state
    let parts = lower_q.trim().split(" ");

    // Words between from and where are from
    // Words between where and select are where
    // Rest is select
    let first = parts.clone().collect::<Vec<_>>()[0];
    if first == Clause::From.as_str() { // Then LQS query
        let mut select_clause = String::from("");
        let mut from_clause = String::from("");
        let mut where_clause = String::from("");

        let mut state = ParserState::new();

        let keywords = [
            Clause::Select.as_str(),
            Clause::From.as_str(),
            Clause::Where.as_str(),
        ];

        let mut in_string = false;

        for (i, part) in parts.into_iter().enumerate() {
            // If word starts with paren, then subquery
            if part.starts_with("(") {
                let parts_vec: Vec<&str> = lower_q.trim().split(" ").collect();
                let sub_parts = parts_vec.clone()[i..].to_vec();
                let mut lqs_subquery = String::from(" ");
                for sub_part in sub_parts {
                    let mut check_part = sub_part.clone();
                    if check_part.starts_with("(") {
                        let mut chars = check_part.chars();
                        chars.next();
                        check_part = chars.as_str();
                    }
                    if check_part.ends_with(")") {
                        let mut chars = check_part.chars();
                        chars.next_back();
                        check_part = chars.as_str();
                        lqs_subquery.push_str(check_part);
                        lqs_subquery.push_str(" ");
                        break;
                    }
                    lqs_subquery.push_str(check_part);
                    lqs_subquery.push_str(" ");
                }
                let mut chars = lqs_subquery.chars();
                chars.next();
                chars.next_back();
                lqs_subquery = chars.as_str().to_string();

                // and end indices
                // But then skip all those parts maybe change state then change back
                let mut subquery = String::from("(");
                subquery += &parse(lqs_subquery);
                subquery += ")";

                if state.current_clause == Clause::Select {
                    select_clause = select_clause + " " + &subquery;
                } else if state.current_clause == Clause::From {
                    from_clause = from_clause + " " + &subquery;
                } else if state.current_clause == Clause::Where {
                    where_clause = where_clause + " " + &subquery;
                }

                state.set(Clause::Skip);
            }

            if part.ends_with(")") && state.current_clause == Clause::Skip {
                state.set(state.last_clause);
            } else if state.current_clause != Clause::Skip {
                // If word starts with quote and doesn't end in quote then ignore keywords until end of quote
                if part.starts_with("\"") && !part.ends_with("\"") {
                    in_string = true;
                } else if in_string == true && part.ends_with("\"") {
                    in_string = false;
                }

                if in_string == false && keywords.contains(&part) {
                    // Change state
                    if part == Clause::Select.as_str() {
                        state.set(Clause::Select);
                    } else if part == Clause::From.as_str() {
                        state.set(Clause::From);
                    } else if part == Clause::Where.as_str() {
                        state.set(Clause::Where);
                    }
                }

                if state.current_clause == Clause::Select {
                    select_clause = select_clause + " " + part.clone();
                } else if state.current_clause == Clause::From {
                    from_clause = from_clause + " " + part.clone();
                } else if state.current_clause == Clause::Where {
                    where_clause = where_clause + " " + part.clone();
                }
            }
        }

        let mut real_query = select_clause.to_owned();
        real_query += " ";
        real_query += &from_clause;
        real_query += " ";
        real_query += &where_clause;
        return real_query;
    } else {
        return lower_q;
    }
}

#[cfg(test)]
mod tests {
    use crate::lqs::parse;

    #[test]
    fn basic_query() {
        let query = String::from("from table where id > 5 select *");
        let result = parse(query);
        let expected = String::from(" select *  from table  where id > 5");
        assert_eq!(result, expected);
    }

    #[test]
    fn normal_basic_query() {
        let query = String::from("select * from table where id > 5");
        let result = parse(query);
        let expected = String::from("select * from table where id > 5");
        assert_eq!(result, expected);
    }

    #[test]
    fn semicolon_end_query() {
        let query = String::from("from table where id > 5 select *;");
        let result = parse(query);
        let expected = String::from(" select *  from table  where id > 5");
        assert_eq!(result, expected);
    }

    #[test]
    fn subquery_query() {
        let query = String::from(
            "from table join table2 on i =i where id in (from event select id) select *",
        );
        let result = parse(query);
        let expected = String::from(
            " select *  from table join table2 on i =i  where id in ( select id  from event )",
        );
        assert_eq!(result, expected);
    }
}
