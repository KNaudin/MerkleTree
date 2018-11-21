mod node;
mod data;
mod tree;

use data::*;
use node::*;
use tree::*;

fn main() {

    let first_node = Node::new(Data::new(String::from("My age"), DataType::Integer(21)), String::from("Added my age."));
    let second_node = Node::new(Data::new(String::from("A float"), DataType::Float(3.14)), String::from("Defined PI."));
    let third_node = Node::new(Data::new(String::from("This is a"), DataType::String(String::from("Working tree."))), String::from("Commented line"));
    let mut fourth_node = Node::new(Data::new(String::from("Random number"), DataType::Integer(21)), String::from("BRANCH ERROR"));

    let mut merkle_tree = Tree::new(first_node);
    merkle_tree.push(second_node, String::from("master"));
    merkle_tree.push(third_node, String::from("master"));

    merkle_tree.print_all_data(String::from("master"));

    let error = merkle_tree.push(fourth_node.clone(), String::from("dev")); //error
    println!("{}", error);

    merkle_tree.new_branch(fourth_node, String::from("dev"));
    merkle_tree.print_all_data(String::from("dev"));
}
