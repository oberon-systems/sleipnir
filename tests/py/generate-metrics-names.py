import json
import time
from pathlib import Path
from random import randint


def generate_metric_names(count: int = 1000, flush_interval: int = 5) -> None:
    """
    Generate uniq metrics names, flash it each interval.
    """

    result: set = set()

    circuits: list = ['live', 'stage', 'dev']
    applications: list = ['web', 'api', 'db', 'cache', 'queue', 'worker', 'lb', 'storage']
    metrics: list = [
        'cpu.usage',
        'cpu.idle',
        'cpu.iowait',
        'cpu.system',
        'cpu.user',
        'memory.used',
        'memory.free',
        'memory.cached',
        'memory.buffers',
        'memory.available',
        'disk.usage',
        'disk.iops.read',
        'disk.iops.write',
        'disk.latency.read',
        'disk.latency.write',
        'disk.throughput.read',
        'disk.throughput.write',
        'network.bytes_in',
        'network.bytes_out',
        'network.packets_in',
        'network.packets_out',
        'network.errors_in',
        'network.errors_out',
        'requests.total',
        'requests.success',
        'requests.errors',
        'requests.rate',
        'response_time.avg',
        'response_time.min',
        'response_time.max',
        'response_time.p50',
        'response_time.p95',
        'response_time.p99',
        'connections.active',
        'connections.idle',
        'connections.total',
        'queue.size',
        'queue.processing',
        'queue.pending',
        'cache.hits',
        'cache.misses',
        'cache.hit_rate',
        'db.queries',
        'db.slow_queries',
        'db.connections',
        'errors.rate',
        'errors.5xx',
        'errors.4xx',
        'users.active',
        'users.registered',
        'orders.total',
        'orders.completed',
        'revenue.total',
        'conversions.rate'
    ]

    max_instances = 1000
    max_possible = len(metrics) * len(circuits) * len(applications) * max_instances

    if count > max_possible:
        print(f"WARNING: Requested {count} metrics, but max possible is {max_possible}")
        print(f"Generating {max_possible} metrics instead...")
        count = max_possible

    last_flush_time = time.time()
    last_flush_count = 0
    start_time = time.time()

    while len(result) < count:
        metric: str = (
            f"{metrics[randint(0, len(metrics) - 1)]}"
            f";circuit={circuits[randint(0, len(circuits) - 1)]}"
            f";application={applications[randint(0, len(applications) - 1)]}"
            f";instance={randint(1, max_instances)}"
        )
        result.add(metric)

        current_time = time.time()

        if current_time - last_flush_time >= flush_interval:
            elapsed = current_time - start_time
            current_count = len(result)
            speed = (current_count - last_flush_count) / flush_interval

            print(
                f"[{elapsed:.1f}s] Generated: {current_count}/{count} "
                f"({current_count / count * 100:.1f}%) | "
                f"Speed: {speed:.0f} metrics/sec"
            )

            save_atomic(list(result), 'metrics.json')

            last_flush_time = current_time
            last_flush_count = current_count

    save_atomic(list(result), 'metrics.json')

    elapsed = time.time() - start_time
    print(f"Generated {len(result)} metrics in {elapsed:.1f} seconds!")


def save_atomic(data: list | dict, file: str) -> None:
    """
    Save data atomic as a json (pretty).
    """
    _tmp = f'{file}.tmp'
    with open(_tmp, 'w+', encoding='utf-8') as handler:
        json.dump(
            data, handler,
            indent=2,
            ensure_ascii=False,
            separators=(',', ': '),
        )
    Path(_tmp).rename(file)


if __name__ == '__main__':
    print('Generation starting...')
    generate_metric_names(600000, flush_interval=5)
    print('Finished')
