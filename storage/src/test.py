import requests
import time


def get_latest():
    return requests.get('http://localhost:3000/transaction/latest')


def add_transaction(trans_id):
    return requests.post(
        'http://localhost:3000/transaction',
        json={
            'id': trans_id,
            'facts': [
                {'entity': trans_id, 'attribute': 'time', 'value': str(int(time.time())), 'transaction': trans_id}
            ]
        }
    )

latest = get_latest()
if not latest.text:
    transaction_id = 1
else:
    transaction_id = latest.json()['id']
add_transaction(transaction_id + 1)
print(get_latest().text)
