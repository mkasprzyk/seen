import binascii
import unittest
import logging

from ephemeral_id_python import *


logging.basicConfig(level=logging.DEBUG)


class TestEphemeralID(unittest.TestCase):
    def test_resolver(self):
        beacon_public_key = binascii.unhexlify(
            '5f2ff6357762b9c188343259a9bd899a9a667d170143c0bc1ae905e877914a0e'
        )

        private_key = binascii.unhexlify(
            '4041414141414141414141414141414141414141414141414141414141414141'
        )

        public_key = binascii.unhexlify(
            '7a1a4e709bf085ac494aba0469b9b1eda0ab1f78b16aabb79ffeda90623e8522'
        )

        scaler = 0
        counter = 0

        shared_secret = compute_shared_secret(private_key, beacon_public_key)
        self.assertEqual(shared_secret, binascii.unhexlify('80722c34967ab7d613c5549224c662aed7cdf5369ec051bcede788a4a29b7677'))

        identity_key = compute_identity_key(shared_secret, public_key, beacon_public_key)
        self.assertEqual(identity_key, binascii.unhexlify('7c91330e61dfea4606b5b3ecb4457d76'))

        computed_eid = compute_ephemeral_id(identity_key, scaler, counter)
        self.assertEqual(computed_eid, binascii.unhexlify('436bfe57d5a09505'))
