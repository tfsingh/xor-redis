import time
import random
import string
import redis
import seaborn as sns
import matplotlib.pyplot as plt
import pandas as pd


def generate_random_strings(num, length=10):
    return [''.join(random.choices(string.ascii_lowercase, k=length)) for _ in range(num)]

def benchmark_xor_filter(redis_conn, num_entries):
    redis_conn.flushdb()
    entries = generate_random_strings(num_entries)

    start_time = time.time()
    redis_conn.execute_command('XOR.POPULATE', 'xor_filter', *entries)
    insert_time = time.time() - start_time

    start_time = time.time()
    for entry in entries:
        redis_conn.execute_command('XOR.CONTAINS', 'xor_filter', entry)
    query_time = time.time() - start_time

    return insert_time, query_time

def benchmark_bloom_filter(redis_conn, num_entries, error_rate=0.01):
    redis_conn.flushdb()
    entries = generate_random_strings(num_entries)

    redis_conn.execute_command('BF.RESERVE', 'bloom_filter', str(error_rate), str(num_entries))

    start_time = time.time()
    for entry in entries:
        redis_conn.execute_command('BF.ADD', 'bloom_filter', entry)
    insert_time = time.time() - start_time

    start_time = time.time()
    for entry in entries:
        redis_conn.execute_command('BF.MEXISTS', 'bloom_filter', *entries)
    query_time = time.time() - start_time

    return insert_time, query_time


def run_benchmarks():
    results = []
    for num_entries in [100, 1000]:
        xor_insert, xor_query = benchmark_xor_filter(redis.StrictRedis(host='localhost', port=6379, db=0), num_entries)
        bloom_insert, bloom_query = benchmark_bloom_filter(redis.StrictRedis(host='localhost', port=6380, db=0), num_entries)

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

    blue_palette = sns.color_palette("Blues_r", n_colors=2)

    sns.lineplot(x='Entries', y='InsertTime', hue='Filter', style='Filter', markers=True, dashes=False, data=df, palette=blue_palette, ax=axes[0])
    axes[0].set_title('Insertion Time')
    axes[0].set_xlabel('Entries')
    axes[0].set_ylabel('Insert Time (s)')
    axes[0].legend(loc='upper left', title='Filter')

    sns.lineplot(x='Entries', y='QueryTime', hue='Filter', style='Filter', markers=True, dashes=False, data=df, palette=blue_palette, ax=axes[1])
    axes[1].set_title('Query Time')
    axes[1].set_xlabel('Entries')
    axes[1].set_ylabel('Query Time (s)')
    axes[1].legend(loc='upper left', title='Filter')

    plt.tight_layout()
    plt.show()

if __name__ == "__main__":
    df_results = run_benchmarks()
    plot_results(df_results)
