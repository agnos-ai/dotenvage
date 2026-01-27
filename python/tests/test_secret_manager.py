"""Tests for SecretManager class."""

import pytest

import dotenvage


class TestSecretManagerGenerate:
    """Tests for SecretManager.generate()."""

    def test_generate_creates_valid_manager(self):
        """Test that generate() creates a working SecretManager."""
        manager = dotenvage.SecretManager.generate()
        assert manager is not None

    def test_generate_creates_unique_keys(self):
        """Test that generate() creates unique key pairs."""
        manager1 = dotenvage.SecretManager.generate()
        manager2 = dotenvage.SecretManager.generate()
        assert manager1.public_key_string() != manager2.public_key_string()

    def test_public_key_has_age_format(self):
        """Test that public key starts with age1."""
        manager = dotenvage.SecretManager.generate()
        public_key = manager.public_key_string()
        assert public_key.startswith("age1")


class TestSecretManagerEncryption:
    """Tests for encryption/decryption."""

    def test_encrypt_creates_wrapped_format(self):
        """Test that encrypt_value creates ENC[AGE:b64:...] format."""
        manager = dotenvage.SecretManager.generate()
        encrypted = manager.encrypt_value("test-secret")
        assert encrypted.startswith("ENC[AGE:b64:")
        assert encrypted.endswith("]")

    def test_decrypt_returns_original(self):
        """Test that decrypt_value returns the original plaintext."""
        manager = dotenvage.SecretManager.generate()
        original = "my-secret-password"
        encrypted = manager.encrypt_value(original)
        decrypted = manager.decrypt_value(encrypted)
        assert decrypted == original

    def test_decrypt_unencrypted_returns_unchanged(self):
        """Test that decrypt_value returns unencrypted values unchanged."""
        manager = dotenvage.SecretManager.generate()
        plain = "not-encrypted"
        result = manager.decrypt_value(plain)
        assert result == plain

    def test_encrypt_empty_string(self):
        """Test encrypting an empty string."""
        manager = dotenvage.SecretManager.generate()
        encrypted = manager.encrypt_value("")
        decrypted = manager.decrypt_value(encrypted)
        assert decrypted == ""

    def test_encrypt_unicode(self):
        """Test encrypting unicode characters."""
        manager = dotenvage.SecretManager.generate()
        original = "password123!"
        encrypted = manager.encrypt_value(original)
        decrypted = manager.decrypt_value(encrypted)
        assert decrypted == original


class TestSecretManagerIsEncrypted:
    """Tests for is_encrypted static method."""

    def test_is_encrypted_true_for_encrypted_value(self):
        """Test that is_encrypted returns True for encrypted values."""
        manager = dotenvage.SecretManager.generate()
        encrypted = manager.encrypt_value("secret")
        assert dotenvage.SecretManager.is_encrypted(encrypted) is True

    def test_is_encrypted_false_for_plain_value(self):
        """Test that is_encrypted returns False for plain values."""
        assert dotenvage.SecretManager.is_encrypted("plain-text") is False
        assert dotenvage.SecretManager.is_encrypted("") is False
        assert dotenvage.SecretManager.is_encrypted("ENC[WRONG:format]") is False


class TestSecretManagerFromIdentity:
    """Tests for from_identity_string factory."""

    def test_from_identity_string_rejects_invalid(self):
        """Test that from_identity_string rejects invalid identity strings."""
        with pytest.raises(ValueError):
            dotenvage.SecretManager.from_identity_string("invalid-identity")
