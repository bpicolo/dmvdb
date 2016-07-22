import requests
import time


transaction_id = 3


def get_latest():
    return requests.get('http://localhost:3000/transaction/latest').text


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


resp = add_transaction(transaction_id)
add_transaction(transaction_id + 1)
print(get_latest())
