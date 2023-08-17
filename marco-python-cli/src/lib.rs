use pyo3::types::PyString;
use pyo3::{prelude::*, types::PyModule};

pub fn marco_python(input: &str) -> PyResult<String> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let marco = PyModule::from_code(
            py,
            r#"
def marco(input):
    if input == "marco":
        return "python"
    else:
        return "no python"
"#,
            "marco.py",
            "marco",
        )?;
        let marco_func = marco.getattr("marco")?;
        let marco_result = marco_func.call1((input,))?;
        let marco_result: &PyString = marco_result.extract()?;
        Ok(marco_result.to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marco_python() {
        let input = "marco";
        let expected_output = "python".to_string();
        let output = marco_python(input).unwrap();
        assert_eq!(output, expected_output, "Failed for input: {}", input);
    }
    
    #[test]
    fn test_no_python() {
        let input = "not_marco";
        let expected_output = "no python".to_string();
        let output = marco_python(input).unwrap();
        assert_eq!(output, expected_output, "Failed for input: {}", input);
    }
}

