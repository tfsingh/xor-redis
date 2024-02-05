import redis
import pytest
import random
import string

@pytest.fixture
def redis_connection():
    conn = redis.StrictRedis(host='localhost', port=6379, db=0)
    conn.flushdb() 
    yield conn
    conn.flushdb()  

def test_xor_populate(redis_connection):
    redis_connection.execute_command('XOR.POPULATE', 'filter', 'entry', 'entry1', 'entry2', 'entry3')

    assert redis_connection.execute_command('XOR.CONTAINS', 'filter', 'entry') == 1
    assert redis_connection.execute_command('XOR.CONTAINS', 'filter', 'entry2') == 1
    assert redis_connection.execute_command('XOR.CONTAINS', 'filter', 'entry1') == 1
    assert redis_connection.execute_command('XOR.CONTAINS', 'filter', 'entry3') == 1
    assert redis_connection.execute_command('XOR.CONTAINS', 'filter', 'not_in') == 0 # not deterministic

def test_stress_test(redis_connection):
    num_entries = 30
    entries = [generate_random_string() for _ in range(num_entries)]

    redis_connection.execute_command('XOR.POPULATE', 'stress_filter', *entries)

    for entry in entries:
        assert redis_connection.execute_command('XOR.CONTAINS', 'stress_filter', entry) == 1

    assert redis_connection.execute_command('XOR.CONTAINS', 'stress_filter', 'not_in') == 0 # not deterministic


def generate_random_string(length=10):
    letters = string.ascii_lowercase
    return ''.join(random.choice(letters) for _ in range(length))

if __name__ == "__main__":
    pytest.main()
