advent_of_code::solution!(9);
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockType {
    Filled(usize),
    Empty,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BlockAllocation {
    length: usize,
    block_type: BlockType,
}

fn consolidate_blocks(filesystem: Vec<BlockAllocation>) -> Vec<BlockAllocation> {
    let mut next_fs = Vec::new();
    let mut last_block = BlockAllocation {
        block_type: BlockType::Empty,
        length: 0,
    };
    for item in filesystem {
        if item.block_type == BlockType::Empty && last_block.block_type == BlockType::Empty {
            last_block.length += item.length;
        } else {
            next_fs.push(last_block);
            last_block = item;
        }
    }
    next_fs.push(last_block);
    next_fs
}

pub fn part_one(input: &str) -> Option<u64> {
    // parse input
    let mut disk_map: Vec<BlockAllocation> = Vec::new();
    let mut current_block_id = 0;
    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            disk_map.push(BlockAllocation {
                length: c.to_digit(10).unwrap() as usize,
                block_type: BlockType::Filled(current_block_id),
            });
            current_block_id += 1;
        } else {
            disk_map.push(BlockAllocation {
                length: c.to_digit(10).unwrap() as usize,
                block_type: BlockType::Empty,
            });
        }
    }

    let mut degragmented_disk = Vec::new();
    let mut available_blocks = 0;

    let mut disk_map = VecDeque::from(disk_map);

    'bigger: while let Some(mut next_file) = disk_map.pop_back() {
        match next_file.block_type {
            BlockType::Filled(_) => {
                if available_blocks >= next_file.length {
                    degragmented_disk.push(next_file);
                    available_blocks -= next_file.length;
                } else {
                    let next_fragment = BlockAllocation {
                        length: available_blocks,
                        block_type: next_file.block_type,
                    };
                    degragmented_disk.push(next_fragment);
                    next_file.length -= available_blocks;
                    available_blocks = 0;
                    while next_file.length > 0 {
                        match disk_map.pop_front() {
                            None => {
                                degragmented_disk.push(next_file);
                                break 'bigger;
                            }
                            Some(b) if b.block_type == BlockType::Empty => {
                                available_blocks += b.length;
                                if available_blocks >= next_file.length {
                                    available_blocks -= next_file.length;
                                    degragmented_disk.push(next_file);
                                    next_file.length = 0;
                                } else {
                                    let next_fragment = BlockAllocation {
                                        length: b.length,
                                        block_type: next_file.block_type,
                                    };
                                    degragmented_disk.push(next_fragment);
                                    next_file.length -= b.length;
                                    available_blocks -= b.length;
                                }
                            }
                            Some(b) => {
                                degragmented_disk.push(b);
                            }
                        }
                    }
                }
            }
            BlockType::Empty => {}
        }
    }

    // score
    let mut position = 0;
    let mut result = 0;
    for item in degragmented_disk {
        if let BlockType::Filled(id) = item.block_type {
            for _ in 0..item.length {
                result += position * id;
                position += 1;
            }
        } else {
            position += item.length
        }
    }

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<usize> {
    // parse input
    let mut disk_map: Vec<BlockAllocation> = Vec::new();
    let mut current_block_id = 0;
    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            disk_map.push(BlockAllocation {
                length: c.to_digit(10).unwrap() as usize,
                block_type: BlockType::Filled(current_block_id),
            });
            current_block_id += 1;
        } else {
            disk_map.push(BlockAllocation {
                length: c.to_digit(10).unwrap() as usize,
                block_type: BlockType::Empty,
            });
        }
    }
    let mut unmovable = Vec::new();
    while let Some(next_file) = disk_map.pop() {
        match next_file.block_type {
            BlockType::Filled(_n) => {
                let mut next_map: Vec<BlockAllocation> = Vec::new();
                let mut placed = false;
                for mut entry in disk_map.iter().copied() {
                    match entry.block_type {
                        BlockType::Empty if !placed => {
                            if entry.length >= next_file.length {
                                next_map.push(next_file);
                                unmovable.push(BlockAllocation {
                                    block_type: BlockType::Empty,
                                    length: next_file.length,
                                });
                                entry.length -= next_file.length;
                                placed = true;
                            }
                            if entry.length != 0 {
                                next_map.push(entry);
                            }
                        }
                        _ => {
                            next_map.push(entry);
                        }
                    }
                }
                if !placed {
                    unmovable.push(next_file);
                }

                disk_map = consolidate_blocks(next_map);
            }
            BlockType::Empty => {
                unmovable.push(next_file);
            }
        }
    }
    unmovable.reverse();

    // score
    let mut position = 0;
    let mut result = 0;
    for item in unmovable {
        if let BlockType::Filled(id) = item.block_type {
            for _ in 0..item.length {
                result += position * id;
                position += 1;
            }
        } else {
            position += item.length
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
