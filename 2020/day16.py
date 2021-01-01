from utils import *


@dataclass
class TicketData:
    fields: List[Tuple]
    your: Tuple
    nearby: List[Tuple]


class Parser:
    def parse(self, fields, your, nearby):
        return TicketData(
            self.parse_fields(fields),
            self.parse_your_ticket(your),
            self.parse_nearby_tickets(nearby))

    def parse_fields(self, section):
        fields = []
        for field_spec in section.split('\n'):
            field, ranges = field_spec.split(':')
            nums = list(map(int, re.findall(r'[0-9]+', ranges)))
            values = set(n for i in range(0, len(nums), 2) for n in range(nums[i], nums[i+1] + 1))
            fields.append((field, values))
        return fields

    def parse_your_ticket(self, section):
        return self.parse_ticket(section.split('\n')[1])

    def parse_nearby_tickets(self, section):
        tickets = section.split('\n')[1:]
        return [self.parse_ticket(t) for t in tickets]

    def parse_ticket(self, ticket):
        return tuple(map(int, ticket.split(',')))


def day16_1(ticket_data):
    valid_vals = set.union(*(vals for _, vals in ticket_data.fields))
    return sum(x for ticket in ticket_data.nearby for x in ticket if x not in valid_vals)


def day16_2(ticket_data):
    def eliminate(pos, field_id):
        if field_id not in positions[pos]:
            return
        positions[pos].remove(field_id)
        if len(positions[pos]) == 1:
            for i in range(len(positions)):
                if i != pos:
                    eliminate(i, positions[pos][0])

    fields, your, nearby = ticket_data.fields, ticket_data.your, ticket_data.nearby
    valid_vals = set.union(*(vals for _, vals in ticket_data.fields))
    valid_nearby = [ticket for ticket in nearby if all(x in valid_vals for x in ticket)]
    field_ids = list(range(len(fields)))
    positions = [field_ids[:] for _ in range(len(fields))]
    for i, col in enumerate(zip(*valid_nearby)):
        for j, (_, vals) in enumerate(fields):
            if any(x not in vals for x in col):
                eliminate(i, j)
    assert {len(x) for x in positions} == {1}
    return product(n for ([id], n) in zip(positions, your)
                     if fields[id][0].startswith('departure'))


if __name__ == "__main__":
    parser = Parser()
    ticket_data = parser.parse(*data(16, str, '\n\n'))
    print(f'day16_1: {day16_1(ticket_data)}')
    print(f'day16_2: {day16_2(ticket_data)}')
    # day16_1: 26980
    # day16_2: 3021381607403
    # python3 day16.py  0.05s user 0.01s system 90% cpu 0.069 total
