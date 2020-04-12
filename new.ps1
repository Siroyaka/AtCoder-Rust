<#

#>
param (
    [ValidateSet("create", "add", "open")]
    [string]$command,
    [string]$contestName,
    [array]$questionNames
)
$LOGFILE = Join-Path $PSScriptRoot "new.ps1.log";
$TEMPLATESOLUPATH = Join-Path $PSScriptRoot "SolutionTemplate\";
$TEMPLATEPROJPATH = Join-Path $PSScriptRoot "ProjectTemplate\";
$PROGRAMFILENAME = "src/main.rs";
$ENTRYPOINT_ROW = "82";

<#
    ## 内容

    ログファイルに日付を付与して情報を書き込む。
#>
function WriteLogFile([string]$logMessage) {
    $nowDate = Get-Date -Format "yyyy/MM/dd HH:mm:ss.f";
    Write-Output "$($nowDate) $($logMessage)" | Add-Content $LOGFILE;
    Write-Host $logMessage;
}

<#
    ## 内容

    VSCodeを開く
#>
function OpenVsCode([string]$directoryPath) {
    $filepath = Join-Path $directoryPath $PROGRAMFILENAME;
    $lineplus = $filepath + ":" + $ENTRYPOINT_ROW;
    code $directoryPath -r -g $lineplus;
}

<#
    ## 内容

    テンプレートが存在する場合行き先のフォルダを上書きする。
#>
function TemplateUpdate([string]$templatePath, [string]$destination) {
    if (Test-Path $templatePath) {
        if ((Get-ChildItem $templatePath).Length -gt 0) {
            Copy-Item -Path "$($templatePath)*" -Destination $destination -Recurse -Force;
            return $true;
        }
    }
    return $false;
}

<#
    ## 内容

    プロジェクトを作成して、もしテンプレートが存在するならばそれで上書きする。
#>
function MakeProject([string]$questionName, [string]$templatePath) {
    # プロジェクトを作成する
    Start-Process -FilePath cargo -ArgumentList new, --bin, $questionName -Wait -WindowStyle Hidden;
    WriteLogFile "Create Project [$($questionName)]";

    # プロジェクトフォルダをテンプレートで上書きする
    if (TemplateUpdate $templatePath $questionName) {
        WriteLogFile "Update $($questionName) to Template";
    }
}

<#
    ## 概要

    コンテストフォルダおよびその中の問題のフォルダを作成する。

    ## 条件

    1. コンテストフォルダが存在しない。
    2. questionNamesで問題名が1つ以上指定されている。

    ## 内容

    - コンテスト名を使用してdotnet new slnコマンドでソリューション作成。
    - コンテストのフォルダ内で問題名を使用してF#のプロジェクトを作成。
    - 作成したプロジェクトをソリューションに結び付ける。
#>
function CreateContest([string]$contestPath) {
    WriteLogFile "CreateContest";

    # すでにコンテストフォルダが作られている場合は終了
    if (Test-Path $contestPath) {
        WriteLogFile "コンテスト名[$($contestName)]が既に存在します。";
        return;
    }

    # questionNameが空
    if ($questionNames.Length -eq 0) {
        WriteLogFile "問題名が空です。";
        return;
    }

    # コンテストのフォルダを作成する。
    New-Item $contestPath -ItemType "Directory" | Out-Null;
    WriteLogFile "Create ContestFolder [$($contestName)]";

    # コンテストのフォルダをテンプレートで上書きする
    if (TemplateUpdate $TEMPLATESOLUPATH $contestPath) {
        WriteLogFile "Update solution directory to Template";
    }

    # ディレクトリを変更する。
    Set-Location $contestPath | Out-Null;
    
    # 問題のプロジェクトを作成する。
    foreach ($questionName in $questionNames) {
        MakeProject $questionName $TEMPLATEPROJPATH;
    }
    WriteLogFile "作成問題数: $($questionNames.Length)";

    OpenVsCode $questionNames[0];

    Set-Location $PSScriptRoot;
}

<#
    ## 概要

    対象のコンテストフォルダに問題のフォルダを追加する。

    ## 条件

    1. コンテストフォルダが存在する。
    2. questionNamesで問題名が1つ以上指定されている。

    ## 内容

    - 対象にしている追加フォルダのうちコンテストフォルダ内に存在しない問題名に対して以下の操作を行う
        1. rustのプロジェクトフォルダを作成する。
        2. ソリューションに作成したプロジェクトを追加する。
#>
function AddQuestion([string]$contestPath) {
    WriteLogFile "AddQuestion";

    # コンテストフォルダが存在しない
    if (!(Test-Path $contestPath)) {
        WriteLogFile "コンテスト名[$($contestName)]が存在しません。";
        return;
    }

    # questionNameが空の場合は作成するフォルダがないため終了
    if ($questionNames.Length -eq 0) {
        WriteLogFile "問題名が空です。";
        return;
    }

    # ディレクトリを変更する。
    Set-Location $contestPath | Out-Null;
    
    $madeQuestionNames = @();
    # 問題のプロジェクトを作成し、ソリューションと結びつける。
    foreach ($questionName in $questionNames) {
        if (Test-Path(Join-Path $contestPath $questionName)) {
            WriteLogFile "コンテストフォルダ内に[$($questionName)]が既に存在します。";
            continue;
        }
        MakeProject $questionName $TEMPLATEPROJPATH;
        $madeQuestionNames += $questionName;
    }
    WriteLogFile "作成問題数: $($madeQuestionNames.Length)";
    
    OpenVsCode $madeQuestionNames[0];

    Set-Location $PSScriptRoot;
}

function Main {
    # ログフォルダの作成(存在していない場合)
    if (!(Test-Path $LOGFILE)) {
        New-Item -Path $LOGFILE -ItemType File | Out-Null;
    }

    # コンテストフォルダのフルパスを作成
    $contestFullPath = Join-Path $PSScriptRoot $contestName;
    switch ($command.ToLower()) {
        "create" {
            CreateContest $contestFullPath;
            break;
        }
        "add" {
            AddQuestion $contestFullPath;
            break;
        }
        "open" {
        }
        default {
            Write-Host "コマンド[$($command)]は有効ではありません。";
            break;
        }
    }

    WriteLogFile "Finish";
    WriteLogFile "--------------------------------";
}

Main;
