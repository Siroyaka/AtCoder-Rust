$pathVec = ($PSScriptRoot -split '\\');
$folderName = $pathVec[$pathVec.Length - 1]
$exePath = ".\target\debug\$($folderName).exe"
$testlog = "testlog.log";
$script:args = $args;
if (!(Test-Path $testlog)) {
    New-Item -Path $testlog -ItemType File | Out-Null;
}

function ExeTest([string]$inputArg) {
    return $inputArg | & $exePath;
}

function ReWriteSameLine([string]$private:message) {
    Write-Output "`r$($message)" | Write-Host -NoNewLine;
}

function LogLine {
    Write-Output "------------------------------------------" | Add-Content $testlog;
}

function OpTestlog([string]$message) {
    Write-Output $message | Add-Content $testlog;
}

function Test([string]$input) {
    OpTestlog "input:";
    OpTestlog $input;
    OpTestlog "";

    $res = ExeTest $input;
    OpTestlog "Answer:";
    OpTestlog $res;
    OpTestlog "";
    return $res;
}

function RandomNumLine([int]$minimum, [int]$maximum, [int]$count) {
    $max = $maximum + 1;
    [string]$res = Get-Random -Minimum $minimum -Maximum $max;
    for ($index = 1; $index -lt $count; $index++) {
        $res += " ";
        $res += Get-Random -Minimum $minimum -Maximum $max;
    }
    return $res;
}

function RandomArraySelectLine([array]$arr, [int]$count, [bool]$isSpace) {
    $space = "";
    if ($isSpace) {
        $space = " ";
    }
    $res = $arr[(Get-Random -Minimum 0 -Maximum $arr.Length)];
    for ($i = 1; $i -lt $count; $i++) {
        $res += $space;
        $res += $arr[(Get-Random -Minimum 0 -Maximum $arr.Length)];
    }
    return $res;
}

function Alphabet([char]$startChar, [char]$endChar) {
    [array]$local:chars = @();
    for ($i = [int]$startChar; $i -le [int]$endChar; $i++) {
        $chars += @([char]$i);
    }
    return $chars;
}

function RandomTest {
    $inputs = "";
    $ans = Test $inputs;
}

function main() {
    [int]$testCount = 1;
    if ($script:args.Length -ne 0) {
        $testCount = [int]$script:args[0]
    }

    Cargo build | Out-null;
    OpTestlog "-TestStart-";
    OpTestlog "Date: $(Get-Date -Format "yyyy/MM/dd HH:mm:ss.f")";

    for ($i = 0; $i -lt $testCount; $i++) {
        LogLine;
        OpTestlog "TestNo:$($i + 1)";
        ReWriteSameLine "Test Count:$($i + 1)";
        RandomTest;
        OpTestlog "";
    }

    Write-Output "";
    OpTestlog "-TestEnd-";
    OpTestlog "";
}

main;
