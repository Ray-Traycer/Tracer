use rand::Rng;
use std::cmp::Ordering;

use std::fmt;

use crate::objects::object::{Bounded, Geometry, ObjectType};
use crate::utils::aabb::surrounding_box;
use crate::world::physics::{Intersection, Ray};

use super::aabb::Aabb;

pub struct BvhTree<'a> {
    nodes: Vec<BvhNode<'a>>,
    root: NodeId,
}

struct BvhNode<'a> {
    left: Option<NodeId>,
    right: Option<NodeId>,
    aabb: Option<Aabb>,
    object: Option<&'a ObjectType>,
}

type NodeId = usize;

impl<'a> BvhTree<'a> {
    pub fn intersects_id(
        &self,
        id: NodeId,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<Intersection> {
        let node = &self.nodes[id];
        if let Some(aabb) = node.aabb {
            if aabb.hit(ray, t_min, t_max) {
                if let Some(obj) = node.object {
                    return obj.intersects(ray, t_min, t_max);
                }

                let hit_left = match node.left {
                    Some(id) => self.intersects_id(id, ray, t_min, t_max),
                    None => None,
                };

                let hit_right = match node.right {
                    Some(id) => self.intersects_id(id, ray, t_min, t_max),
                    None => None,
                };

                return match (hit_left, hit_right) {
                    (None, None) => None,
                    (None, hit) => hit,
                    (hit, None) => hit,
                    (Some(left), Some(right)) => {
                        if left.distance < right.distance {
                            Some(left)
                        } else {
                            Some(right)
                        }
                    }
                };
            }
        }

        None
    }
}

impl<'a> BvhTree<'a> {
    pub fn new(l: &'a mut Vec<ObjectType>) -> BvhTree<'a> {
        let mut tree = BvhTree {
            nodes: Vec::new(),
            root: 0,
        };
        tree.root = tree.build(l);

        tree
    }

    fn build(&mut self, l: &'a mut [ObjectType]) -> NodeId {
        let axis = rand::thread_rng().gen_range(0..3);

        match axis {
            0 => l.sort_by(|a, b| box_x_compare(a, b)),
            1 => l.sort_by(|a, b| box_y_compare(a, b)),
            2 => l.sort_by(|a, b| box_z_compare(a, b)),
            _ => panic!("Unexpected axis"),
        }

        let left: NodeId;
        let right: NodeId;

        if l.len() == 1 {
            return self.new_leaf(&l[0]);
        } else if l.len() == 2 {
            left = self.new_leaf(&l[0]);
            right = self.new_leaf(&l[1]);
        } else {
            let half_len = l.len() / 2;
            let (left_hitables, right_hitables) = l.split_at_mut(half_len);

            left = self.build(left_hitables);
            right = self.build(right_hitables);
        }

        if let Some(left_box) = self.nodes[left].aabb {
            if let Some(right_box) = self.nodes[right].aabb {
                return self.new_node(
                    surrounding_box(&left_box, &right_box),
                    Some(left),
                    Some(right),
                );
            }
        }

        panic!("No bounding box in BvhNode::build");
    }

    fn new_leaf(&mut self, object: &'a ObjectType) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(BvhNode {
            left: None,
            right: None,
            aabb: object.bounding_box(),
            object: Some(object),
        });

        return next_index;
    }

    fn new_node(&mut self, aabb: Aabb, left: Option<NodeId>, right: Option<NodeId>) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(BvhNode {
            left,
            right,
            aabb: Some(aabb),
            object: None,
        });

        return next_index;
    }

    fn number_hittables(&self, id: NodeId) -> usize {
        let node = &self.nodes[id];
        let local_hitable = if node.object.is_some() { 1 } else { 0 };
        let count_left = if let Some(left_index) = node.left {
            self.number_hittables(left_index)
        } else {
            0
        };
        let count_right = if let Some(right_index) = node.right {
            self.number_hittables(right_index)
        } else {
            0
        };

        local_hitable + count_left + count_right
    }

    fn bounding_box(&self) -> Option<Aabb> {
        self.nodes[self.root].aabb
    }

    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<Intersection> {
        self.intersects_id(self.root, r, tmin, tmax)
    }
}

impl<'a> fmt::Display for BvhTree<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BVH with {:?} hitables and {:?} nodes",
            self.number_hittables(self.root),
            self.nodes.len()
        )
    }
}

fn box_x_compare(a: &ObjectType, b: &ObjectType) -> Ordering {
    if let Some(box_left) = a.bounding_box() {
        if let Some(box_right) = b.bounding_box() {
            if let Some(cmp) = box_left.min.x.partial_cmp(&box_right.min.x) {
                return cmp;
            } else {
                panic!("Can't compare");
            }
        }
    }

    panic!("No bounding box in BvhNode::new");
}

fn box_y_compare(a: &ObjectType, b: &ObjectType) -> Ordering {
    if let Some(box_left) = a.bounding_box() {
        if let Some(box_right) = b.bounding_box() {
            if let Some(cmp) = box_left.min.y.partial_cmp(&box_right.min.y) {
                return cmp;
            } else {
                panic!("Can't compare");
            }
        }
    }

    panic!("No bounding box in BvhNode::new");
}

fn box_z_compare(a: &ObjectType, b: &ObjectType) -> Ordering {
    if let Some(box_left) = a.bounding_box() {
        if let Some(box_right) = b.bounding_box() {
            if let Some(cmp) = box_left.min.z.partial_cmp(&box_right.min.z) {
                return cmp;
            } else {
                panic!("Can't compare");
            }
        }
    }

    panic!("No bounding box in BvhNode::new");
}
