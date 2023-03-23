use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .without_time()
        .init();

    let data = r#"
{
    "status": "ok",
    "filename": "conda",
    "data": [
        {
            "name": "pyenv",
            "ver": "2.2.5-1",
            "repo": "community",
            "path": [
                "/usr/share/pyenv/pyenv.d/exec/pip-rehash/conda"
            ]
        },
        {
            "name": "anaconda",
            "ver": "2021.05-1",
            "repo": "archlinuxcn",
            "path": [
                "/opt/anaconda/bin/conda",
                "/opt/anaconda/condabin/conda",
                "/opt/anaconda/lib/python3.8/site-packages/conda/shell/bin/conda",
                "/opt/anaconda/pkgs/conda-4.10.1-py38h06a4308_1/bin/conda",
                "/opt/anaconda/pkgs/conda-4.10.1-py38h06a4308_1/condabin/conda",
                "/opt/anaconda/pkgs/conda-4.10.1-py38h06a4308_1/lib/python3.8/site-packages/conda/shell/bin/conda"
            ]
        },
        {
            "name": "miniconda",
            "ver": "4.11.0-1",
            "repo": "archlinuxcn",
            "path": [
                "/opt/miniconda/bin/conda",
                "/opt/miniconda/condabin/conda",
                "/opt/miniconda/lib/python3.9/site-packages/conda/shell/bin/conda",
                "/opt/miniconda/pkgs/conda-4.11.0-py39h06a4308_0/bin/conda",
                "/opt/miniconda/pkgs/conda-4.11.0-py39h06a4308_0/condabin/conda",
                "/opt/miniconda/pkgs/conda-4.11.0-py39h06a4308_0/lib/python3.9/site-packages/conda/shell/bin/conda"
            ]
        }
    ]
}
"#;
    let result = jsonom::parse(data).unwrap();
    println!("{result:#?}");
}
