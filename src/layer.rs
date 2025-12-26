use super::*;

pub struct Layer<Node> {
    pub node: Node,
}

impl Layer<()> {
    pub fn new<Req>(requirement: Req) -> Layer<Node<Req, (), ()>> {
        Layer {
            node: Node {
                requirement,
                prev_left: (),
                prev_right: (),
            },
        }
    }

    #[allow(dead_code)]
    pub(crate) fn dangerously_new_from_node<Node>(node: Node) -> Layer<Node> {
        Layer { node }
    }
}

impl<Node> Layer<Node> {
    pub fn get<Target, Direction>(&self) -> &Target
    where
        Node: Get<Target, Direction>,
    {
        self.node.get()
    }

    pub fn get_mut<Target, Direction>(&mut self) -> &mut Target
    where
        Node: Get<Target, Direction>,
    {
        self.node.get_mut()
    }

    pub fn insert<NewReq, MinDepth, NextPath, Next>(self, new_req: NewReq) -> Layer<Next>
    where
        MinDepth: IsNumber,
        Node: Insert<NewReq, MinDepth, NextPath, Next>,
    {
        Layer {
            node: self.node.insert(new_req),
        }
    }
}
