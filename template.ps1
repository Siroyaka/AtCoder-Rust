param([string]$contestName, [int]$questionNumber)

if ($questionNumber -gt 26) {
    Write-Output "QuestionCount is Under 26";
    return;
}

if ($questionNumber -lt 1) {
    Write-Output "QuestionCount is Over 1";
    return;
}

$questionNames = @((0..($questionNumber - 1)) | ForEach-Object {"question_$([char]([int][char]'a' + $_))"});

.\new.ps1 Create $contestName $questionNames;