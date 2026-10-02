import pytest

pytest.importorskip("pytest_pyodide")

from pathlib import Path

from pytest_pyodide.decorator import SeleniumType, copy_files_to_pyodide

ROOT = Path(__file__).resolve().parent.parent


@copy_files_to_pyodide(
    [(ROOT / "dist" / "crc32c_rs-*.whl", Path())],
    install_wheels=True,
)
def test_crc32c(selenium: SeleniumType) -> None:
    selenium.run_async("""
import array
import unittest

from crc32c_rs import crc32c

GIL_MINSIZE = 32 * 1024


class Crc32c(unittest.TestCase):
    def test_buffer(self):
        for data, expected in [
            (b"", 0),
            (b"123456789", 0xE3069283),
            (bytearray(b"123456789"), 0xE3069283),
            (memoryview(b"123456789"), 0xE3069283),
            (array.array("B", b"123456789"), 0xE3069283),
        ]:
            self.assertEqual(crc32c(data), expected)

    def test_memoryview_slice(self):
        data = b"a" * 32
        expected = crc32c(data[10:26])
        mv = memoryview(data)[10:26]
        self.assertEqual(crc32c(mv), expected)
        self.assertEqual(len(mv), 16)

    def test_not_a_buffer(self):
        for bad in (12345, None, {"key": "value"}):
            with self.assertRaises(TypeError):
                crc32c(bad)

    def test_sizes(self):
        for size, expected in [
            (GIL_MINSIZE - 1, 0x6EC6DE93),
            (GIL_MINSIZE, 0x40468A0D),
            (GIL_MINSIZE + 1, 0x7EB84969),
            (GIL_MINSIZE * 2, 0x4E95ED3F),
            (31, 0xE269F709),
            (32, 0xB980F10B),
            (33, 0x58E068FA),
            (63, 0x029FDBC5),
            (64, 0x37AEEE33),
            (65, 0xE254579B),
            (127, 0x162B309A),
            (128, 0x388CBC2F),
            (129, 0xBF467D76),
            (255, 0xF0A023DE),
            (256, 0xBE8BBD9F),
            (257, 0x0DD508BE),
            (512, 0x6F32A61F),
            (1000, 0x9F19EF6A),
            (1023, 0x28259078),
            (1024, 0x3AB96A62),
            (1025, 0x411719CF),
        ]:
            self.assertEqual(crc32c(b"a" * size), expected)


suite = unittest.TestLoader().loadTestsFromTestCase(Crc32c)
result = unittest.TextTestRunner(verbosity=2).run(suite)
assert result.wasSuccessful()
""")
