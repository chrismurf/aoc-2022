use std::fs;
use pyo3::prelude::*;

fn main() -> PyResult<()> {
    let input = fs::read_to_string("input.txt").unwrap();

    Python::with_gil(|py| {
        let module: Py<PyModule> = PyModule::from_code(py, "
def part1(input):
    return max([sum(map(int, x.split('\\n'))) for x in input[:-1].split('\\n\\n')])
                
def part2(input):
    return sum(sorted([sum(map(int, x.split('\\n'))) for x in input[:-1].split('\\n\\n')])[-3:])
", "", "")?.into();
        let part1 : Py<PyAny> = module.getattr(py, "part1")?.into();
        let part2 : Py<PyAny> = module.getattr(py, "part2")?.into();

        println!("Part 1: {}", part1.call1(py, (&input,))?.extract::<i64>(py)?);
        println!("Part 2: {}", part2.call1(py, (&input,))?.extract::<i64>(py)?);
        Ok(())
    })
}
