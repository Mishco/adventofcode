test = """vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"""

total_priority = 0
content = open('../inputs/day03.txt').read()

for item in content.splitlines():
    first_half = item[0:len(item) // 2]
    second_half = item[len(item) // 2:len(item)]
    #print(first_half, second_half)

    matches = set(first_half) & set(second_half)
    #print(matches)
    # items = set(first_half + second_half)
    priorities = [ord(item) - 96 if item.islower() else ord(item) - 38 for item in matches]
    #print(priorities)
    total_priority += sum(priorities)


print("Total priority:", total_priority) # 7737

rucksacks = content.split("\n")
badge_priorities = []

for i in range(0, len(rucksacks), 3):
    group_rucksacks = rucksacks[i:i+3]
    items = set(group_rucksacks[0])
    for rucksack in group_rucksacks[1:]:
        items &= set(rucksack)
    if len(items) == 1:
        badge = items.pop()
        priority = ord(badge) - 96 if badge.islower() else ord(badge) - 38
        badge_priorities.append(priority)

# print("Badge priorities:", badge_priorities)
print("Sum of priorities:", sum(badge_priorities))
