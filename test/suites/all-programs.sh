#Spell test suite

# Test cases

function TestAdd() {
    local base_dir="$(dirname "${0}")"
    local asm_file="${base_dir}/../../resources/asm/Add.asm"
    local hack_out="${base_dir}/../../resources/hack/Add.hack"
    local hack_out_validation="${base_dir}/../expected/Add.hack"

    ./"${base_dir}"/../../target/release/hack-assembler -i "${asm_file}" -o "${hack_out}" \
        && diff -q "${hack_out}" "${hack_out_validation}"
}
TEST_CASES["1"]="TestAdd"

function TestMax() {
    local base_dir="$(dirname "${0}")"
    local asm_file="${base_dir}/../../resources/asm/Max.asm"
    local hack_out="${base_dir}/../../resources/hack/Max.hack"
    local hack_out_validation="${base_dir}/../expected/Max.hack"

    ./"${base_dir}"/../../target/release/hack-assembler -i "${asm_file}" -o "${hack_out}" \
        && diff -q "${hack_out}" "${hack_out_validation}"
}
TEST_CASES["2"]="TestMax"

function TestRect() {
    local base_dir="$(dirname "${0}")"
    local asm_file="${base_dir}/../../resources/asm/Rect.asm"
    local hack_out="${base_dir}/../../resources/hack/Rect.hack"
    local hack_out_validation="${base_dir}/../expected/Rect.hack"

    ./"${base_dir}"/../../target/release/hack-assembler -i "${asm_file}" -o "${hack_out}" \
        && diff -q "${hack_out}" "${hack_out_validation}"
}
TEST_CASES["3"]="TestRect"

function TestPong() {
    local base_dir="$(dirname "${0}")"
    local asm_file="${base_dir}/../../resources/asm/Pong.asm"
    local hack_out="${base_dir}/../../resources/hack/Pong.hack"
    local hack_out_validation="${base_dir}/../expected/Pong.hack"

    ./"${base_dir}"/../../target/release/hack-assembler -i "${asm_file}" -o "${hack_out}" \
        && diff -q "${hack_out}" "${hack_out_validation}"
}
TEST_CASES["4"]="TestPong"

function TestInvalidProgram() {
    local base_dir="$(dirname "${0}")"
    local asm_file="${base_dir}/../../resources/asm/invalid.asm"
    local hack_out="${base_dir}/../../resources/hack/invalid.hack"

    ./"${base_dir}"/../../target/release/hack-assembler -i "${asm_file}" -o "${hack_out}" 2 > /dev/null
    [[ $? -ne 0 ]]
}
TEST_CASES["5"]="TestInvalidProgram"

export TEST_CASES
