pub struct Operation<I, O> {
    client_id: usize,
    input: I,
    call: i64,
    output: O,
    ret: i64,
}

type EventKind = bool;

const CALL_EVENT: EventKind = false;
const RET_EVENT: EventKind = true;

pub struct Event<T> {
    client_id: usize,
    kind: EventKind,
    value: T,
    id: usize,
}

trait Model<I, O, T, S> {
    fn partition(&self, history: &[Operation<I, O>]) -> Vec<Vec<Operation<I, O>>>;
    fn partition_event(&self, history: &[Event<T>]) -> Vec<Vec<Event<T>>>;
    fn init(&self) -> bool;
    fn step(&self, state: S, input: I, output: O) -> (bool, I);
    fn equal(&self, state1: S, state2: S) -> bool;
    fn describe_operation(&self, input: I, output: O) -> String;
    fn describe_state(&self, state: S) -> String;
}
