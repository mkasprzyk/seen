#!/usr/bin/env python2

import binascii
import hashlib
import logging

from Crypto.Cipher import AES
import nacl.bindings
import hkdf


logger = logging.getLogger(__name__)


def compute_shared_secret(private_key, public_key):
    logger.debug('public_key: {}'.format(binascii.hexlify(public_key)))
    logger.debug('private_key: {}'.format(private_key))

    shared_secret = nacl.bindings.crypto_scalarmult(private_key, public_key)
    logger.debug('shared_secret: {}'.format(binascii.hexlify(shared_secret)))

    return shared_secret

def compute_identity_key(shared_secret, service_public_key, beacon_public_key):
    logger.debug('service_public_key: {}'.format(binascii.hexlify(service_public_key)))
    logger.debug('beacon_public_key: {}'.format(binascii.hexlify(beacon_public_key)))

    salt = service_public_key + beacon_public_key
    prk = hkdf.hkdf_extract(salt, shared_secret, hash=hashlib.sha256)

    identity_key = hkdf.hkdf_expand(prk, b"", 32, hash=hashlib.sha256)[:16]
    logger.debug('identity_key: {}'.format(binascii.hexlify(identity_key)))

    return identity_key

def compute_temporary_key(identity_key, counter):
    temporary_key_data = (
      "\x00" * 11 +
      "\xFF" +
      "\x00" * 2 +
      chr((counter // (2 ** 24)) % 256) +
      chr((counter // (2 ** 16)) % 256))

    temporary_key = AES.new(identity_key, AES.MODE_ECB).encrypt(temporary_key_data)
    logger.debug('temporary_key: {}'.format(binascii.hexlify(temporary_key)))

    return temporary_key

def compute_ephemeral_id(identity_key, scaler, counter):
    rotation_exponent = 2 ** scaler
    logger.debug('rotation_exponent: {}'.format(rotation_exponent))

    beacon_time_seconds = (counter // rotation_exponent) * rotation_exponent
    logger.debug('beacon_time_seconds: {}'.format(beacon_time_seconds))

    ephemeral_id_data = (
        "\x00" * 11 +
        chr(scaler) +
        chr((beacon_time_seconds // (2 ** 24)) % 256) +
        chr((beacon_time_seconds // (2 ** 16)) % 256) +
        chr((beacon_time_seconds // (2 ** 8)) % 256) +
        chr((beacon_time_seconds // (2 ** 0)) % 256))

    temporary_key = compute_temporary_key(identity_key, counter)
    logger.debug("ephemeral_id_data: {}".format(binascii.hexlify(ephemeral_id_data)))

    ephemeral_id = AES.new(temporary_key, AES.MODE_ECB).encrypt(ephemeral_id_data)[:8]
    logger.debug("ephemeral_id: {}".format(binascii.hexlify(ephemeral_id)))

    return ephemeral_id
