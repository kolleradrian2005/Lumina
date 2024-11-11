$p = Get-Process lumina

while ($true) {

	((Get-Counter "\GPU Process Memory(pid_$($p.id)*)\Local Usage").CounterSamples | where CookedValue).CookedValue | 
foreach {Write-Output "Process $($P.Name) GPU Process Memory $([math]::Round($_/1MB,2)) MB"}
	((Get-Counter "\GPU Engine(pid_$($p.id)*engtype_3D)\Utilization Percentage").CounterSamples | where CookedValue).CookedValue |
foreach {Write-Output "Process $($P.Name) GPU Engine Usage $([math]::Round($_,2))%"}

}