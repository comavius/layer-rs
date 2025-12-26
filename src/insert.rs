use super::*;

/// This requires that the target requirement does not already exist in the tree.
pub trait Insert<NewReq, MinDepth, Queries, NotHasKind, Next>
where
    MinDepth: IsNumber,
    Self::MinDepth: IsNumber,
    Self: NotHas<NewReq, NotHasKind>,
{
    /// Inserts a new requirement into the tree.
    /// If you want to replace an existing requirement, `remove` it beforehand.
    fn insert(self, new_req: NewReq) -> Next;
    type MinDepth;
    type Queries;
}

// insert: insert into leaf node
impl<Req, NewReq, NotHasKind>
    Insert<NewReq, Succ<Zero>, Left<Here>, NotHasKind, Node<Req, Node<NewReq, (), ()>, ()>>
    for Node<Req, (), ()>
where
    Self: NotHas<NewReq, NotHasKind>,
{
    fn insert(self, new_req: NewReq) -> Node<Req, Node<NewReq, (), ()>, ()> {
        Node {
            requirement: self.requirement,
            prev_left: Node {
                requirement: new_req,
                prev_left: (),
                prev_right: (),
            },
            prev_right: (),
        }
    }

    type MinDepth = Succ<Zero>;
    type Queries = Left<Here>;
}

// insert: insert into left-null tree
impl<Req, ReqR, NewReq, NotHasKind>
    Insert<
        NewReq,
        Succ<Succ<Zero>>,
        Left<Right<Here>>,
        NotHasKind,
        Node<Req, Node<NewReq, (), ()>, Node<ReqR, (), ()>>,
    > for Node<Req, (), Node<ReqR, (), ()>>
where
    Self: NotHas<NewReq, NotHasKind>,
{
    fn insert(self, new_req: NewReq) -> Node<Req, Node<NewReq, (), ()>, Node<ReqR, (), ()>> {
        Node {
            requirement: self.requirement,
            prev_left: Node {
                requirement: new_req,
                prev_left: (),
                prev_right: (),
            },
            prev_right: self.prev_right,
        }
    }

    type MinDepth = Succ<Succ<Zero>>;
    type Queries = Left<Here>;
}

// insert: insert into right-null tree
impl<Req, ReqL, NewReq, NotHasKind>
    Insert<
        NewReq,
        Succ<Succ<Zero>>,
        Right<Left<Here>>,
        NotHasKind,
        Node<Req, Node<ReqL, (), ()>, Node<NewReq, (), ()>>,
    > for Node<Req, Node<ReqL, (), ()>, ()>
where
    Self: NotHas<NewReq, NotHasKind>,
{
    fn insert(self, new_req: NewReq) -> Node<Req, Node<ReqL, (), ()>, Node<NewReq, (), ()>> {
        Node {
            requirement: self.requirement,
            prev_left: self.prev_left,
            prev_right: Node {
                requirement: new_req,
                prev_left: (),
                prev_right: (),
            },
        }
    }

    type MinDepth = Succ<Succ<Zero>>;
    type Queries = Right<Here>;
}

// insert: recurse into left subtree twice
impl<
    Req,
    ReqL,
    SubtreeLL,
    SubtreeLR,
    ReqR,
    SubtreeRL,
    SubtreeRR,
    NewReq,
    NextSubtreeL,
    PrevQueriesL,
    NotHasKind,
>
    Insert<
        NewReq,
        <Node<Req, NextSubtreeL, Node<ReqR, SubtreeRL, SubtreeRR>> as MinDepth>::Output,
        Left<PrevQueriesL>,
        NotHasKind,
        Node<Req, NextSubtreeL, Node<ReqR, SubtreeRL, SubtreeRR>>,
    > for Node<Req, Node<ReqL, SubtreeLL, SubtreeLR>, Node<ReqR, SubtreeRL, SubtreeRR>>
where
    Node<ReqL, SubtreeLL, SubtreeLR>: MinDepth,
    Node<ReqR, SubtreeRL, SubtreeRR>: MinDepth,
    Node<Req, NextSubtreeL, Node<ReqR, SubtreeRL, SubtreeRR>>: MinDepth,
    NextSubtreeL: MinDepth,
    Node<ReqL, SubtreeLL, SubtreeLR>:
        Insert<NewReq, <NextSubtreeL as MinDepth>::Output, PrevQueriesL, NotHasKind, NextSubtreeL>,
    (): LessThan<
            <Node<ReqL, SubtreeLL, SubtreeLR> as MinDepth>::Output,
            <Node<ReqR, SubtreeRL, SubtreeRR> as MinDepth>::Output,
        >,
    Self: NotHas<NewReq, NotHasKind>,
{
    fn insert(self, new_req: NewReq) -> Node<Req, NextSubtreeL, Node<ReqR, SubtreeRL, SubtreeRR>> {
        Node {
            requirement: self.requirement,
            prev_left: self.prev_left.insert(new_req),
            prev_right: self.prev_right,
        }
    }

    type MinDepth =
        Succ<<Node<Req, NextSubtreeL, Node<ReqR, SubtreeRL, SubtreeRR>> as MinDepth>::Output>;
    type Queries = Left<PrevQueriesL>;
}

// insert: recurse into left subtree twice
impl<
    Req,
    ReqL,
    SubtreeLL,
    SubtreeLR,
    ReqR,
    SubtreeRL,
    SubtreeRR,
    NewReq,
    NextSubtreeR,
    PrevQueriesR,
    NotHasKind,
>
    Insert<
        NewReq,
        <Node<Req, Node<ReqL, SubtreeLL, SubtreeLR>, NextSubtreeR> as MinDepth>::Output,
        Right<PrevQueriesR>,
        NotHasKind,
        Node<Req, Node<ReqL, SubtreeLL, SubtreeLR>, NextSubtreeR>,
    > for Node<Req, Node<ReqL, SubtreeLL, SubtreeLR>, Node<ReqR, SubtreeRL, SubtreeRR>>
where
    Node<ReqL, SubtreeLL, SubtreeLR>: MinDepth,
    Node<ReqR, SubtreeRL, SubtreeRR>: MinDepth,
    Node<Req, Node<ReqL, SubtreeLL, SubtreeLR>, NextSubtreeR>: MinDepth,
    NextSubtreeR: MinDepth,
    Node<ReqR, SubtreeRL, SubtreeRR>:
        Insert<NewReq, <NextSubtreeR as MinDepth>::Output, PrevQueriesR, NotHasKind, NextSubtreeR>,
    (): GreaterThanOrEqual<
            <Node<ReqL, SubtreeLL, SubtreeLR> as MinDepth>::Output,
            <Node<ReqR, SubtreeRL, SubtreeRR> as MinDepth>::Output,
        >,
    Self: NotHas<NewReq, NotHasKind>,
{
    fn insert(self, new_req: NewReq) -> Node<Req, Node<ReqL, SubtreeLL, SubtreeLR>, NextSubtreeR> {
        Node {
            requirement: self.requirement,
            prev_left: self.prev_left,
            prev_right: self.prev_right.insert(new_req),
        }
    }

    type MinDepth =
        Succ<<Node<Req, Node<ReqL, SubtreeLL, SubtreeLR>, NextSubtreeR> as MinDepth>::Output>;
    type Queries = Right<PrevQueriesR>;
}
