import os
import json
import socket
import time
import logging
from random import randint, uniform
from datetime import datetime, timedelta

logging.basicConfig(
        format='[%(asctime)s] %(name)s[%(process)d][%(levelname)s]: %(message)s',
        level=os.getenv('APP_LOG_LEVEL', 'INFO').upper(),
    )

log = logging.getLogger('generator')


class GraphiteMetricGenerator:
    """
    generator class
    """

    def __init__(self, host='localhost', port=2003):
        self.host = host
        self.port = port
        self.sock = None

    def connect(self):
        """
        Conector
        """
        try:
            self.sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.sock.connect((self.host, self.port))
            log.info(f"connected {self.host}:{self.port}")
            return True
        except Exception as e:
            log.error(f"connection error: {e}")
            return False

    def send_metric(self, metrics: bytes):
        """
        Sender
        """
        try:
            if self.sock:
                self.sock.sendall(metrics)
                return True
            else:
                log.error('not connected')
                return False
        except Exception as e:
            log.error(f"transmit error: {e}")
            return False

    def close(self):
        """
        Cleanup
        """
        if self.sock:
            self.sock.close()
            log.info('connection closed')


def read_metrics_list() -> dict | list:
    """
    Read json file content.
    """
    with open('metrics.json', encoding='utf-8') as handler:
        data = json.load(handler)
    return data


def random_ts(start: datetime, end: datetime) -> int:
    """
    Get random ts in a range.
    """
    delta = end - start
    random_seconds = randint(0, int(delta.total_seconds()))
    return int((start + timedelta(seconds=random_seconds)).timestamp())


if __name__ == '__main__':

    graphite_host = os.getenv('APP_HOST', 'localhost')
    graphite_port = int(os.getenv('APP_PORT', 2003))

    metrics = read_metrics_list()
    generator = GraphiteMetricGenerator(graphite_host, graphite_port)

    start = datetime(2025, 1, 1)
    end = datetime(2025, 2, 20)

    if generator.connect():

        while True:
            metric = metrics[randint(0, len(metrics) - 1)]
            value = uniform(0, 1000)
            ts = random_ts(start, end)

            message = f"{metric} {value:.2f} {ts}\n"
            generator.send_metric(message.encode('utf-8'))

    else:
        log.error('unable to connect')
