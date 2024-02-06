import time
import random
import string
import redis
import seaborn as sns
import matplotlib.pyplot as plt
import pandas as pd

def generate_random_strings(num, length=10):
    return [''.join(random.choices(string.ascii_lowercase, k=length)) for _ in range(num)]

def benchmark_xor_filter(redis_conn, entries):
    redis_conn.flushdb()

    start_time = time.time()
    redis_conn.execute_command('XOR.POPULATE', 'xor_filter', *entries)
    insert_time = time.time() - start_time

    start_time = time.time()
    for entry in entries:
        redis_conn.execute_command('XOR.CONTAINS', 'xor_filter', entry)
    query_time = time.time() - start_time

    return insert_time, query_time

def benchmark_bloom_filter(redis_conn, entries, error_rate=0.01):
    redis_conn.flushdb()

    redis_conn.execute_command('BF.RESERVE', 'bloom_filter', str(error_rate), str(len(entries)))

    start_time = time.time()
    redis_conn.execute_command('BF.MADD', 'bloom_filter', *entries)
    insert_time = time.time() - start_time

    start_time = time.time()
    for entry in entries:
        redis_conn.execute_command('BF.EXISTS', 'bloom_filter', entry)
    query_time = time.time() - start_time

    return insert_time, query_time


def run_benchmarks():
    results = []
    for num_entries in [100, 500, 1000, 5000, 10000, 25000, 50000]:
        entries = generate_random_strings(num_entries)
        xor_insert, xor_query = benchmark_xor_filter(redis.StrictRedis(host='localhost', port=6379, db=0), entries)
        bloom_insert, bloom_query = benchmark_bloom_filter(redis.StrictRedis(host='localhost', port=6380, db=0), entries)

        results.append({
            'Entries': num_entries,
            'Filter': 'XOR',
            'InsertTime': xor_insert,
            'QueryTime': xor_query
        })

        results.append({
            'Entries': num_entries,
            'Filter': 'Bloom',
            'InsertTime': bloom_insert,
            'QueryTime': bloom_query
        })

        print("DONE WITH", num_entries)

    return pd.DataFrame(results)

def plot_results(df):
    sns.set(style="whitegrid", rc={"axes.titlesize":16,"axes.labelsize":14})
    fig, axes = plt.subplots(1, 2, figsize=(14, 6))

    sns.lineplot(x='Entries', y='InsertTime', hue='Filter', style='Filter', markers=True, dashes=False, data=df, ax=axes[0])
    axes[0].set_title('Insertion')
    axes[0].set_xlabel('Entries')
    axes[0].set_ylabel('Execution Time (s)')
    axes[0].legend(loc='upper left', title='Filter')

    sns.lineplot(x='Entries', y='QueryTime', hue='Filter', style='Filter', markers=True, dashes=False, data=df, ax=axes[1])
    axes[1].set_title('Querying')
    axes[1].set_xlabel('Entries')
    axes[1].set_ylabel('Execution Time (s)')
    axes[1].legend(loc='upper left', title='Filter')

    plt.tight_layout()
    plt.show()

if __name__ == "__main__":
    df_results = run_benchmarks()
    plot_results(df_results)
