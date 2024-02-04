#!/bin/bash

#Test suite runner
# arg1: test suite file path

source "$(dirname "${0}")"/logging.sh

LOG_DIR="$(dirname "${0}")/../logs"
LOG_FILE="${LOG_DIR}/test.output"
ERR_FILE="${LOG_DIR}/test.error"
LOG="tee -a ${LOG_FILE}"
LOG_ERR="tee -a ${LOG_FILE} ${ERR_FILE}"

mkdir -p "${LOG_DIR}" && rm -f "${LOG_FILE}" "${ERR_FILE}"

declare -A TEST_CASES
declare -a FAILED_TEST_CASES

# Verify command line args
[[ $# -ne 1 ]] &&
    $ErrMessage "Invalid number of command line arguments" && 
    echo -e "usage: $(basename $0) <test-suite-to-run>\n" \
        "  ex: $(basename $0) test-suite.sh" &&
    exit 1

# Source the test suite
[[ ! -f "${1}" ]] && $ErrMessage "No valid test suite specified" && exit 1
TEST_SUITE="${1}" && shift
source "${TEST_SUITE}"

function Main() {
    local returnCode=0

    IFS=$'\n'
    local sortedTestCases=( $(sort <<< "${!TEST_CASES[*]}") )
    unset IFS

    for testCase in "${sortedTestCases[@]}"; do
        local test="${TEST_CASES[${testCase}]}"
        $TestingMessage "Running ${testCase}::${test}" | ${LOG}

        "${test}"
        local rc=$?

        # Test case result actions
        [[ ${rc} -ne 0 ]] &&
            $FailureMessage "${testCase}::${test} failed with return code: ${rc}" | ${LOG_ERR} &&
            FAILED_TEST_CASES+=("${testCase}::${test}") &&
            returnCode=1 ||
            $TestingSuccessMessage "${testCase}::${test}" | ${LOG}
    done

    # Final test suite report
    [[ ${returnCode} -ne 0 ]] &&
        $ErrMessage "There were failed tests:" | ${LOG_ERR} &&
        printf '%s\n' "${FAILED_TEST_CASES[@]}" | ${LOG_ERR} ||
        $SuccessMessage "All test cases passed" | ${LOG}

    $InfoMessage "Test runner complete"
    exit ${returnCode}
}

Main "$@"

