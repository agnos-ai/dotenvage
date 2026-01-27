"""Type stubs for dotenvage Python bindings."""

from typing import Final

__version__: Final[str]

class SecretManager:
    """Manager for encrypting and decrypting secrets using age encryption."""

    def __init__(self) -> None:
        """Create a SecretManager by loading the key from standard locations.

        Key discovery order:
        0. Auto-discover AGE_KEY_NAME from .env or .env.local files
        1. DOTENVAGE_AGE_KEY environment variable (full identity string)
        2. AGE_KEY environment variable
        3. EKG_AGE_KEY environment variable
        4. Key file from AGE_KEY_NAME ({namespace}/{keyname})
           e.g., myapp/production -> ~/.local/state/myapp/production.key
        5. Default: ~/.local/state/dotenvage/dotenvage.key

        Raises:
            RuntimeError: If no valid key can be found or loaded.
        """
        ...

    @staticmethod
    def generate() -> SecretManager:
        """Generate a new random identity (key pair).

        Returns:
            A new SecretManager with a freshly generated key pair.

        Raises:
            RuntimeError: If key generation fails.
        """
        ...

    @staticmethod
    def from_identity_string(identity: str) -> SecretManager:
        """Create a SecretManager from an existing age identity string.

        Args:
            identity: An age identity string (starts with AGE-SECRET-KEY-).

        Returns:
            A SecretManager using the provided identity.

        Raises:
            ValueError: If the identity string is invalid.
        """
        ...

    @staticmethod
    def is_encrypted(value: str) -> bool:
        """Check if a value is in a recognized encrypted format.

        Args:
            value: The value to check.

        Returns:
            True if the value matches the ENC[AGE:b64:...] format.
        """
        ...

    def public_key_string(self) -> str:
        """Get the public key as a string in age format.

        Returns:
            The public key string (starts with age1).
        """
        ...

    def encrypt_value(self, plaintext: str) -> str:
        """Encrypt a plaintext value.

        Args:
            plaintext: The value to encrypt.

        Returns:
            The encrypted value in ENC[AGE:b64:...] format.

        Raises:
            RuntimeError: If encryption fails.
        """
        ...

    def decrypt_value(self, value: str) -> str:
        """Decrypt a value if it's encrypted, otherwise return unchanged.

        Args:
            value: The value to decrypt (may or may not be encrypted).

        Returns:
            The decrypted plaintext, or the original value if not encrypted.

        Raises:
            RuntimeError: If decryption fails for an encrypted value.
        """
        ...

class EnvLoader:
    """Loader for .env files with automatic decryption of encrypted values."""

    def __init__(self) -> None:
        """Create an EnvLoader with a default SecretManager.

        Raises:
            RuntimeError: If the default SecretManager cannot be created.
        """
        ...

    @staticmethod
    def with_manager(manager: SecretManager) -> EnvLoader:
        """Create an EnvLoader with a specific SecretManager.

        Args:
            manager: The SecretManager to use for decryption.

        Returns:
            An EnvLoader using the provided manager.
        """
        ...

    def load(self) -> list[str]:
        """Load .env files from the current directory.

        Files are loaded in specificity order, with later files overriding
        earlier ones. Encrypted values are automatically decrypted and
        loaded into the process environment.

        Returns:
            List of file paths that were actually loaded, in load order.

        Raises:
            RuntimeError: If loading fails.
        """
        ...

    def load_from_dir(self, dir: str) -> list[str]:
        """Load .env files from a specific directory.

        Args:
            dir: The directory to load .env files from.

        Returns:
            List of file paths that were actually loaded, in load order.

        Raises:
            RuntimeError: If loading fails.
        """
        ...

    def get_all_variable_names(self) -> list[str]:
        """Get all variable names from .env files in the current directory.

        Returns:
            List of variable names defined in .env files.

        Raises:
            RuntimeError: If reading fails.
        """
        ...

    def get_all_variable_names_from_dir(self, dir: str) -> list[str]:
        """Get all variable names from .env files in a specific directory.

        Args:
            dir: The directory to scan for .env files.

        Returns:
            List of variable names defined in .env files.

        Raises:
            RuntimeError: If reading fails.
        """
        ...

    def get_all_variables(self) -> dict[str, str]:
        """Load and return all variables as a dictionary.

        This loads variables into the process environment first, then
        returns them as a dictionary with decrypted values.

        Returns:
            Dictionary mapping variable names to their (decrypted) values.

        Raises:
            RuntimeError: If loading fails.
        """
        ...

    def get_all_variables_from_dir(self, dir: str) -> dict[str, str]:
        """Load and return all variables from a directory as a dictionary.

        Args:
            dir: The directory to load .env files from.

        Returns:
            Dictionary mapping variable names to their (decrypted) values.

        Raises:
            RuntimeError: If loading fails.
        """
        ...

    def resolve_env_paths(self, dir: str) -> list[str]:
        """Compute the ordered list of .env file paths that would be loaded.

        Args:
            dir: The directory to resolve paths for.

        Returns:
            Ordered list of .env file paths (may include non-existent files).
        """
        ...

def should_encrypt(key: str) -> bool:
    """Check if a key name should be encrypted based on auto-detection patterns.

    Keys containing PASSWORD, SECRET, KEY, TOKEN, CREDENTIAL, AUTH, PRIVATE,
    and similar patterns are detected as sensitive.

    Args:
        key: The environment variable name to check.

    Returns:
        True if the key name matches sensitive patterns.
    """
    ...
