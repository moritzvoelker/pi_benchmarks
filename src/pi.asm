DEFAULT REL

section .data
align 32
    divi dq 2.0, 3.0, 4.0, 0.0  ; the three dividents plus one padding
align 32
    incr dq 2.0, 2.0, 2.0, 0.0  ; the times the increment plus one padding
    quot dq 4.0                 ; quotient
    pi   dq 3.0                 ; the result beginning with three


section .text
    global pi_asm
pi_asm:
    push rbp                ; stack framey stuff
    mov rbp, rsp

    movsd xmm0, [pi]        ; loading pi accumulator

    xor rcx, rcx            ; setting up loop counter
loop:
    cmp rcx, rdi            ; loop for number of iterations
    je endloop

    movsd xmm2, [divi]      ; multiply dividents
    movsd xmm3, [divi + 8]
    movsd xmm4, [divi + 16]
    mulsd xmm2, xmm3
    mulsd xmm2, xmm4

    movsd xmm1, [quot]      ; divide quotient by product of dividents
    divsd xmm1, xmm2

    addsd xmm0, xmm1        ; add to the accumulator

    vmovapd ymm1, [divi]     ; increment dividents
    vmovapd ymm2, [incr]
    vaddpd ymm1, ymm2
    vmovapd [divi], ymm1

    movsd xmm2, [divi]      ; multiply dividents
    movsd xmm3, [divi + 8]
    movsd xmm4, [divi + 16]
    mulsd xmm2, xmm3
    mulsd xmm2, xmm4

    ; do the same thing twice, but the second time we subtract from the accumulator

    movsd xmm1, [quot]      ; divide quotient by product of dividents
    divsd xmm1, xmm2

    subsd xmm0, xmm1        ; subtract from the accumulator

    vmovapd ymm1, [divi]     ; increment dividents
    vmovapd ymm2, [incr]
    vaddpd ymm1, ymm2
    vmovapd [divi], ymm1

    inc rcx                 ; increment loop counter
    jmp loop
endloop:

    mov rsp, rbp            ; restore stack framey stuff
    pop rbp
    ret