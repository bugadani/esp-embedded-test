param (
    [string[]]$targets
)

# esp32s2 is missing because serde-json-core is not compatible with it yet
$validTargets = @("esp32", "esp32s3", "esp32c2", "esp32c3", "esp32c6", "esp32h2")

mkdir output -ErrorAction SilentlyContinue

if ($targets) {
    foreach ($target in $targets) {
        if (-not $validTargets -contains $target) {
            Write-Error "Invalid target specified: $target. Valid targets are: $($validTargets -join ', ')"
            exit 1
        }
    }
} else {
    $targets = $validTargets
}

# For each target, call cargo
foreach ($target in $targets) {
    echo "Building $target"
    $output = cargo +esp $target --release --test example_test --no-run --message-format=json

    # parse json output of second to last line
    $json = $output[-2] | ConvertFrom-Json
    cp $json.executable "output/$target"
}
