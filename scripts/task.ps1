param (
    [Parameter(Mandatory=$true)]
    [ValidateSet("start", "handoff", "done")]
    [string]$Action,

    [Parameter(Mandatory=$true)]
    [string]$Slug
)

$BaseDir = Get-Location
$WorktreeRootDir = ".." # Use relative path to sibling directory for cross-platform consistency
$TaskDir = Join-Path $WorktreeRootDir "rust-algo-task-$Slug"
$BranchName = "task/$Slug"
$BaseBranch = "release/integrated-trading-platform"

function Update-Board {
    param($NewStatus, $AgentName)
    $BoardPath = Join-Path $BaseDir ".agent\BOARD.md"
    $Content = Get-Content $BoardPath
    $Updated = @()
    foreach ($line in $Content) {
        if ($line -like "*| ``$Slug`` |*") {
            # Update the line while preserving the task description
            # We assume description is the last column
            $Updated += "| ``$Slug`` | $AgentName | $NewStatus | $TaskDir | Investigate and fix Position Manager God Loop |"
        } else {
            $Updated += $line
        }
    }
    $Updated | Out-File $BoardPath -Encoding utf8
}

if ($Action -eq "start") {
    Write-Host "🚀 Starting task: $Slug" -ForegroundColor Cyan
    if (!(Test-Path $TaskDir)) {
        Write-Host "Creating new worktree at $TaskDir from $BaseBranch..."
        git worktree add -b $BranchName $TaskDir $BaseBranch
    } else {
        Write-Host "Found existing worktree at $TaskDir."
    }
    Update-Board "In Progress" "Antigravity"
    Write-Host "✅ Task initialized. Please cd to $TaskDir to begin." -ForegroundColor Green
}
elseif ($Action -eq "handoff") {
    Write-Host "📦 Handing off task: $Slug" -ForegroundColor Yellow
    $HandoffFile = Join-Path $BaseDir ".agent\handoffs\$Slug.md"
    if (!(Test-Path $HandoffFile)) {
        "Please fill in handoff details for $Slug" | Out-File $HandoffFile
    }
    Update-Board "Paused" "None"
    Write-Host "✅ Handoff prepared at $HandoffFile" -ForegroundColor Green
}
elseif ($Action -eq "done") {
    Write-Host "🏁 Finishing task: $Slug" -ForegroundColor Green
    Update-Board "Done" "None"
    Write-Host "Please merge branch $BranchName into $BaseBranch and run git worktree remove $TaskDir manually."
}
