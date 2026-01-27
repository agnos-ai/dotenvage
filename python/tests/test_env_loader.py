"""Tests for EnvLoader class."""

import os
import tempfile

import dotenvage


class TestEnvLoaderBasic:
    """Basic tests for EnvLoader."""

    def test_create_with_manager(self):
        """Test creating EnvLoader with a specific SecretManager."""
        manager = dotenvage.SecretManager.generate()
        loader = dotenvage.EnvLoader.with_manager(manager)
        assert loader is not None

    def test_resolve_env_paths_returns_list(self):
        """Test that resolve_env_paths returns a list."""
        manager = dotenvage.SecretManager.generate()
        loader = dotenvage.EnvLoader.with_manager(manager)

        with tempfile.TemporaryDirectory() as tmpdir:
            paths = loader.resolve_env_paths(tmpdir)
            assert isinstance(paths, list)


class TestEnvLoaderLoading:
    """Tests for loading .env files."""

    def test_load_from_empty_dir(self):
        """Test loading from a directory with no .env files."""
        manager = dotenvage.SecretManager.generate()
        loader = dotenvage.EnvLoader.with_manager(manager)

        with tempfile.TemporaryDirectory() as tmpdir:
            loaded = loader.load_from_dir(tmpdir)
            assert loaded == []

    def test_load_simple_env_file(self):
        """Test loading a simple .env file."""
        manager = dotenvage.SecretManager.generate()
        loader = dotenvage.EnvLoader.with_manager(manager)

        with tempfile.TemporaryDirectory() as tmpdir:
            # Create a simple .env file
            env_path = os.path.join(tmpdir, ".env")
            with open(env_path, "w") as f:
                f.write("TEST_VAR=hello\n")
                f.write("ANOTHER_VAR=world\n")

            loaded = loader.load_from_dir(tmpdir)
            assert len(loaded) == 1
            assert env_path in loaded[0]

            # Use get_all_variables_from_dir to verify values
            variables = loader.get_all_variables_from_dir(tmpdir)
            assert variables.get("TEST_VAR") == "hello"
            assert variables.get("ANOTHER_VAR") == "world"

    def test_load_encrypted_values(self):
        """Test loading .env file with encrypted values."""
        manager = dotenvage.SecretManager.generate()
        loader = dotenvage.EnvLoader.with_manager(manager)

        with tempfile.TemporaryDirectory() as tmpdir:
            # Encrypt a value
            encrypted = manager.encrypt_value("secret-password")

            # Create .env with encrypted value
            env_path = os.path.join(tmpdir, ".env")
            with open(env_path, "w") as f:
                f.write(f"MY_SECRET={encrypted}\n")
                f.write("PLAIN_VAR=visible\n")

            loader.load_from_dir(tmpdir)

            # Use get_all_variables_from_dir to verify decryption happened
            variables = loader.get_all_variables_from_dir(tmpdir)
            assert variables.get("MY_SECRET") == "secret-password"
            assert variables.get("PLAIN_VAR") == "visible"


class TestEnvLoaderVariables:
    """Tests for getting variable information."""

    def test_get_all_variable_names_from_dir(self):
        """Test getting all variable names from a directory."""
        manager = dotenvage.SecretManager.generate()
        loader = dotenvage.EnvLoader.with_manager(manager)

        with tempfile.TemporaryDirectory() as tmpdir:
            env_path = os.path.join(tmpdir, ".env")
            with open(env_path, "w") as f:
                f.write("VAR_ONE=1\n")
                f.write("VAR_TWO=2\n")
                f.write("VAR_THREE=3\n")

            names = loader.get_all_variable_names_from_dir(tmpdir)
            assert "VAR_ONE" in names
            assert "VAR_TWO" in names
            assert "VAR_THREE" in names

    def test_get_all_variables_from_dir(self):
        """Test getting all variables as a dict."""
        manager = dotenvage.SecretManager.generate()
        loader = dotenvage.EnvLoader.with_manager(manager)

        with tempfile.TemporaryDirectory() as tmpdir:
            env_path = os.path.join(tmpdir, ".env")
            with open(env_path, "w") as f:
                f.write("KEY_A=value_a\n")
                f.write("KEY_B=value_b\n")

            variables = loader.get_all_variables_from_dir(tmpdir)
            assert isinstance(variables, dict)
            assert variables.get("KEY_A") == "value_a"
            assert variables.get("KEY_B") == "value_b"
