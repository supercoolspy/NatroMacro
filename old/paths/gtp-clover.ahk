﻿if (MoveMethod = "walk")
{
paths["clover"] := "
(LTrim Join`r`n
;gotoramp
" nm_Walk(47.25, BackKey, LeftKey) "
" nm_Walk(52.5, LeftKey) "
" nm_Walk(2.2, BackKey, RightKey) "
" nm_Walk(6.7, BackKey)"
" nm_Walk(25.5, LeftKey) "
" nm_Walk(35, FwdKey, LeftKey)"
" nm_Walk(7, BackKey, RightKey) "
" nm_Walk(12, RightKey) "
)"
}
else {
paths["clover"] := "
(LTrim Join`r`n
;gotoramp
;gotocannon
send {e down}
HyperSleep(100)
send {e up}{" LeftKey " down}{" FwdKey " down}
HyperSleep(525)
send {space 2}
HyperSleep(1250)
send {" FwdKey " up}
HyperSleep(3850)
send {" LeftKey " up}{space}
HyperSleep(1000)
" nm_Walk(10, FwdKey, LeftKey) "
" nm_Walk(15, LeftKey) "
" nm_Walk(7, FwdKey) "
" nm_Walk(7, BackKey, RightKey) "
" nm_Walk(12, RightKey) "
)"
}
;path 230729 noobyguy