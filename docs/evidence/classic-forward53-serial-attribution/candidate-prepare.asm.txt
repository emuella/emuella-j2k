Disassembly of section .text:

0000000000151600 <emuella_j2k_codestream::scalable_lossless::prepare_forward53>:
  151600:	55                                              	push   %rbp
  151601:	41 57                                           	push   %r15
  151603:	41 56                                           	push   %r14
  151605:	41 55                                           	push   %r13
  151607:	41 54                                           	push   %r12
  151609:	53                                              	push   %rbx
  15160a:	48 81 ec a8 02 00 00                            	sub    $0x2a8,%rsp
  151611:	48 89 d0                                        	mov    %rdx,%rax
  151614:	48 0f af c6                                     	imul   %rsi,%rax
  151618:	48 3d 00 10 00 00                               	cmp    $0x1000,%rax
  15161e:	0f 92 c0                                        	setb   %al
  151621:	48 83 f9 02                                     	cmp    $0x2,%rcx
  151625:	41 0f 92 c0                                     	setb   %r8b
  151629:	41 08 c0                                        	or     %al,%r8b
  15162c:	75 18                                           	jne    151646 <emuella_j2k_codestream::scalable_lossless::prepare_forward53+0x46>
  15162e:	48 83 fe 11                                     	cmp    $0x11,%rsi
  151632:	0f 92 c0                                        	setb   %al
  151635:	48 83 fa 02                                     	cmp    $0x2,%rdx
  151639:	41 0f 92 c0                                     	setb   %r8b
  15163d:	41 08 c0                                        	or     %al,%r8b
  151640:	41 80 f8 01                                     	cmp    $0x1,%r8b
  151644:	75 1d                                           	jne    151663 <emuella_j2k_codestream::scalable_lossless::prepare_forward53+0x63>
  151646:	48 c7 87 b0 00 00 00 ff ff ff ff                	movq   $0xffffffffffffffff,0xb0(%rdi)
  151651:	48 81 c4 a8 02 00 00                            	add    $0x2a8,%rsp
  151658:	5b                                              	pop    %rbx
  151659:	41 5c                                           	pop    %r12
  15165b:	41 5d                                           	pop    %r13
  15165d:	41 5e                                           	pop    %r14
  15165f:	41 5f                                           	pop    %r15
  151661:	5d                                              	pop    %rbp
  151662:	c3                                              	ret
  151663:	48 89 7c 24 08                                  	mov    %rdi,0x8(%rsp)
  151668:	48 83 f9 08                                     	cmp    $0x8,%rcx
  15166c:	41 be 08 00 00 00                               	mov    $0x8,%r14d
  151672:	4c 0f 42 f1                                     	cmovb  %rcx,%r14
  151676:	44 89 f0                                        	mov    %r14d,%eax
  151679:	c1 e0 16                                        	shl    $0x16,%eax
  15167c:	48 39 f2                                        	cmp    %rsi,%rdx
  15167f:	48 89 f1                                        	mov    %rsi,%rcx
  151682:	48 0f 47 ca                                     	cmova  %rdx,%rcx
  151686:	48 8d 0c 49                                     	lea    (%rcx,%rcx,2),%rcx
  15168a:	48 8d 04 88                                     	lea    (%rax,%rcx,4),%rax
  15168e:	48 05 00 f0 ff ff                               	add    $0xfffffffffffff000,%rax
  151694:	48 89 44 24 28                                  	mov    %rax,0x28(%rsp)
  151699:	48 89 f0                                        	mov    %rsi,%rax
  15169c:	48 d1 e8                                        	shr    $1,%rax
  15169f:	48 89 f1                                        	mov    %rsi,%rcx
  1516a2:	48 29 c1                                        	sub    %rax,%rcx
  1516a5:	48 89 d0                                        	mov    %rdx,%rax
  1516a8:	48 d1 e8                                        	shr    $1,%rax
  1516ab:	48 89 d7                                        	mov    %rdx,%rdi
  1516ae:	48 29 c7                                        	sub    %rax,%rdi
  1516b1:	48 89 c8                                        	mov    %rcx,%rax
  1516b4:	48 89 b4 24 d8 00 00 00                         	mov    %rsi,0xd8(%rsp)
  1516bc:	48 89 94 24 e0 00 00 00                         	mov    %rdx,0xe0(%rsp)
  1516c4:	48 89 b4 24 e8 00 00 00                         	mov    %rsi,0xe8(%rsp)
  1516cc:	48 89 8c 24 f0 00 00 00                         	mov    %rcx,0xf0(%rsp)
  1516d4:	48 89 bc 24 f8 00 00 00                         	mov    %rdi,0xf8(%rsp)
  1516dc:	66 c7 84 24 00 01 00 00 00 00                   	movw   $0x0,0x100(%rsp)
  1516e6:	66 c7 84 24 08 01 00 00 01 20                   	movw   $0x2001,0x108(%rsp)
  1516f0:	48 89 8c 24 10 01 00 00                         	mov    %rcx,0x110(%rsp)
  1516f8:	48 d1 e9                                        	shr    $1,%rcx
  1516fb:	48 29 c8                                        	sub    %rcx,%rax
  1516fe:	48 89 f9                                        	mov    %rdi,%rcx
  151701:	48 89 bc 24 18 01 00 00                         	mov    %rdi,0x118(%rsp)
  151709:	48 d1 ef                                        	shr    $1,%rdi
  15170c:	48 29 f9                                        	sub    %rdi,%rcx
  15170f:	48 89 b4 24 20 01 00 00                         	mov    %rsi,0x120(%rsp)
  151717:	48 89 84 24 28 01 00 00                         	mov    %rax,0x128(%rsp)
  15171f:	48 89 8c 24 30 01 00 00                         	mov    %rcx,0x130(%rsp)
  151727:	66 c7 84 24 38 01 00 00 00 00                   	movw   $0x0,0x138(%rsp)
  151731:	66 c7 84 24 40 01 00 00 01 20                   	movw   $0x2001,0x140(%rsp)
  15173b:	48 8d ac 24 50 01 00 00                         	lea    0x150(%rsp),%rbp
  151743:	4c 8d ac 24 00 02 00 00                         	lea    0x200(%rsp),%r13
  15174b:	0f 1f 44 00 00                                  	nopl   0x0(%rax,%rax,1)
  151750:	ba 02 00 00 00                                  	mov    $0x2,%edx
  151755:	41 b8 10 00 00 00                               	mov    $0x10,%r8d
  15175b:	48 8d bc 24 48 01 00 00                         	lea    0x148(%rsp),%rdi
  151763:	48 8d b4 24 d8 00 00 00                         	lea    0xd8(%rsp),%rsi
  15176b:	b9 02 00 00 00                                  	mov    $0x2,%ecx
  151770:	4d 89 f1                                        	mov    %r14,%r9
  151773:	ff 15 9f 3e 12 00                               	call   *0x123e9f(%rip)        # 275618 <_DYNAMIC+0xa68>
  151779:	0f b6 9c 24 f0 01 00 00                         	movzbl 0x1f0(%rsp),%ebx
  151781:	80 fb ff                                        	cmp    $0xff,%bl
  151784:	0f 84 d3 01 00 00                               	je     15195d <emuella_j2k_codestream::scalable_lossless::prepare_forward53+0x35d>
  15178a:	4c 8b a4 24 48 01 00 00                         	mov    0x148(%rsp),%r12
  151792:	4c 8b bc 24 78 01 00 00                         	mov    0x178(%rsp),%r15
  15179a:	48 8b 45 20                                     	mov    0x20(%rbp),%rax
  15179e:	48 89 44 24 50                                  	mov    %rax,0x50(%rsp)
  1517a3:	0f 10 45 00                                     	movups 0x0(%rbp),%xmm0
  1517a7:	0f 10 4d 10                                     	movups 0x10(%rbp),%xmm1
  1517ab:	0f 29 4c 24 40                                  	movaps %xmm1,0x40(%rsp)
  1517b0:	0f 29 44 24 30                                  	movaps %xmm0,0x30(%rsp)
  1517b5:	0f 10 85 90 00 00 00                            	movups 0x90(%rbp),%xmm0
  1517bc:	0f 29 84 24 c0 00 00 00                         	movaps %xmm0,0xc0(%rsp)
  1517c4:	0f 10 85 80 00 00 00                            	movups 0x80(%rbp),%xmm0
  1517cb:	0f 29 84 24 b0 00 00 00                         	movaps %xmm0,0xb0(%rsp)
  1517d3:	0f 10 45 70                                     	movups 0x70(%rbp),%xmm0
  1517d7:	0f 29 84 24 a0 00 00 00                         	movaps %xmm0,0xa0(%rsp)
  1517df:	0f 10 45 30                                     	movups 0x30(%rbp),%xmm0
  1517e3:	0f 10 4d 40                                     	movups 0x40(%rbp),%xmm1
  1517e7:	0f 10 55 50                                     	movups 0x50(%rbp),%xmm2
  1517eb:	0f 10 5d 60                                     	movups 0x60(%rbp),%xmm3
  1517ef:	0f 29 9c 24 90 00 00 00                         	movaps %xmm3,0x90(%rsp)
  1517f7:	0f 29 94 24 80 00 00 00                         	movaps %xmm2,0x80(%rsp)
  1517ff:	0f 29 4c 24 70                                  	movaps %xmm1,0x70(%rsp)
  151804:	0f 29 44 24 60                                  	movaps %xmm0,0x60(%rsp)
  151809:	8b 85 a1 00 00 00                               	mov    0xa1(%rbp),%eax
  15180f:	8b 8d a4 00 00 00                               	mov    0xa4(%rbp),%ecx
  151815:	89 04 24                                        	mov    %eax,(%rsp)
  151818:	89 4c 24 03                                     	mov    %ecx,0x3(%rsp)
  15181c:	4c 89 a4 24 f8 01 00 00                         	mov    %r12,0x1f8(%rsp)
  151824:	48 8b 44 24 50                                  	mov    0x50(%rsp),%rax
  151829:	49 89 45 20                                     	mov    %rax,0x20(%r13)
  15182d:	0f 28 44 24 30                                  	movaps 0x30(%rsp),%xmm0
  151832:	0f 28 4c 24 40                                  	movaps 0x40(%rsp),%xmm1
  151837:	41 0f 11 4d 10                                  	movups %xmm1,0x10(%r13)
  15183c:	41 0f 11 45 00                                  	movups %xmm0,0x0(%r13)
  151841:	4c 89 bc 24 28 02 00 00                         	mov    %r15,0x228(%rsp)
  151849:	0f 28 84 24 c0 00 00 00                         	movaps 0xc0(%rsp),%xmm0
  151851:	41 0f 11 85 90 00 00 00                         	movups %xmm0,0x90(%r13)
  151859:	0f 28 84 24 b0 00 00 00                         	movaps 0xb0(%rsp),%xmm0
  151861:	41 0f 11 85 80 00 00 00                         	movups %xmm0,0x80(%r13)
  151869:	0f 28 84 24 a0 00 00 00                         	movaps 0xa0(%rsp),%xmm0
  151871:	41 0f 11 45 70                                  	movups %xmm0,0x70(%r13)
  151876:	0f 28 44 24 60                                  	movaps 0x60(%rsp),%xmm0
  15187b:	0f 28 4c 24 70                                  	movaps 0x70(%rsp),%xmm1
  151880:	0f 28 94 24 80 00 00 00                         	movaps 0x80(%rsp),%xmm2
  151888:	0f 28 9c 24 90 00 00 00                         	movaps 0x90(%rsp),%xmm3
  151890:	41 0f 11 5d 60                                  	movups %xmm3,0x60(%r13)
  151895:	41 0f 11 55 50                                  	movups %xmm2,0x50(%r13)
  15189a:	41 0f 11 4d 40                                  	movups %xmm1,0x40(%r13)
  15189f:	41 0f 11 45 30                                  	movups %xmm0,0x30(%r13)
  1518a4:	88 9c 24 a0 02 00 00                            	mov    %bl,0x2a0(%rsp)
  1518ab:	8b 04 24                                        	mov    (%rsp),%eax
  1518ae:	8b 4c 24 03                                     	mov    0x3(%rsp),%ecx
  1518b2:	41 89 8d a4 00 00 00                            	mov    %ecx,0xa4(%r13)
  1518b9:	41 89 85 a1 00 00 00                            	mov    %eax,0xa1(%r13)
  1518c0:	84 db                                           	test   %bl,%bl
  1518c2:	0f 84 80 00 00 00                               	je     151948 <emuella_j2k_codestream::scalable_lossless::prepare_forward53+0x348>
  1518c8:	4a 8d 04 bd 00 00 00 00                         	lea    0x0(,%r15,4),%rax
  1518d0:	48 3b 44 24 28                                  	cmp    0x28(%rsp),%rax
  1518d5:	77 58                                           	ja     15192f <emuella_j2k_codestream::scalable_lossless::prepare_forward53+0x32f>
  1518d7:	48 c7 44 24 10 00 00 00 00                      	movq   $0x0,0x10(%rsp)
  1518e0:	48 c7 44 24 18 04 00 00 00                      	movq   $0x4,0x18(%rsp)
  1518e9:	48 c7 44 24 20 00 00 00 00                      	movq   $0x0,0x20(%rsp)
  1518f2:	48 8d bc 24 48 01 00 00                         	lea    0x148(%rsp),%rdi
  1518fa:	48 8d 74 24 10                                  	lea    0x10(%rsp),%rsi
  1518ff:	48 8d 94 24 f8 01 00 00                         	lea    0x1f8(%rsp),%rdx
  151907:	48 8b 4c 24 28                                  	mov    0x28(%rsp),%rcx
  15190c:	ff 15 0e 3d 12 00                               	call   *0x123d0e(%rip)        # 275620 <_DYNAMIC+0xa70>
  151912:	83 bc 24 48 01 00 00 ff                         	cmpl   $0xffffffff,0x148(%rsp)
  15191a:	74 63                                           	je     15197f <emuella_j2k_codestream::scalable_lossless::prepare_forward53+0x37f>
  15191c:	48 83 7c 24 10 00                               	cmpq   $0x0,0x10(%rsp)
  151922:	74 0b                                           	je     15192f <emuella_j2k_codestream::scalable_lossless::prepare_forward53+0x32f>
  151924:	48 8b 7c 24 18                                  	mov    0x18(%rsp),%rdi
  151929:	ff 15 99 34 12 00                               	call   *0x123499(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  15192f:	49 83 fe 02                                     	cmp    $0x2,%r14
  151933:	74 13                                           	je     151948 <emuella_j2k_codestream::scalable_lossless::prepare_forward53+0x348>
  151935:	4c 89 f0                                        	mov    %r14,%rax
  151938:	48 d1 e8                                        	shr    $1,%rax
  15193b:	49 29 c6                                        	sub    %rax,%r14
  15193e:	49 83 fe 01                                     	cmp    $0x1,%r14
  151942:	0f 87 08 fe ff ff                               	ja     151750 <emuella_j2k_codestream::scalable_lossless::prepare_forward53+0x150>
  151948:	48 8b 44 24 08                                  	mov    0x8(%rsp),%rax
  15194d:	48 c7 80 b0 00 00 00 ff ff ff ff                	movq   $0xffffffffffffffff,0xb0(%rax)
  151958:	e9 f4 fc ff ff                                  	jmp    151651 <emuella_j2k_codestream::scalable_lossless::prepare_forward53+0x51>
  15195d:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
  151967:	48 8b 4c 24 08                                  	mov    0x8(%rsp),%rcx
  15196c:	48 89 01                                        	mov    %rax,(%rcx)
  15196f:	48 c7 81 b0 00 00 00 fe ff ff ff                	movq   $0xfffffffffffffffe,0xb0(%rcx)
  15197a:	e9 d2 fc ff ff                                  	jmp    151651 <emuella_j2k_codestream::scalable_lossless::prepare_forward53+0x51>
  15197f:	48 8b 44 24 20                                  	mov    0x20(%rsp),%rax
  151984:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  151989:	48 89 82 c0 00 00 00                            	mov    %rax,0xc0(%rdx)
  151990:	0f 10 44 24 10                                  	movups 0x10(%rsp),%xmm0
  151995:	0f 11 82 b0 00 00 00                            	movups %xmm0,0xb0(%rdx)
  15199c:	48 8b 44 24 50                                  	mov    0x50(%rsp),%rax
  1519a1:	48 89 42 28                                     	mov    %rax,0x28(%rdx)
  1519a5:	0f 28 44 24 30                                  	movaps 0x30(%rsp),%xmm0
  1519aa:	0f 28 4c 24 40                                  	movaps 0x40(%rsp),%xmm1
  1519af:	0f 11 4a 18                                     	movups %xmm1,0x18(%rdx)
  1519b3:	0f 11 42 08                                     	movups %xmm0,0x8(%rdx)
  1519b7:	0f 28 44 24 60                                  	movaps 0x60(%rsp),%xmm0
  1519bc:	0f 28 4c 24 70                                  	movaps 0x70(%rsp),%xmm1
  1519c1:	0f 28 94 24 80 00 00 00                         	movaps 0x80(%rsp),%xmm2
  1519c9:	0f 28 9c 24 90 00 00 00                         	movaps 0x90(%rsp),%xmm3
  1519d1:	0f 11 42 38                                     	movups %xmm0,0x38(%rdx)
  1519d5:	0f 11 4a 48                                     	movups %xmm1,0x48(%rdx)
  1519d9:	0f 11 52 58                                     	movups %xmm2,0x58(%rdx)
  1519dd:	0f 11 5a 68                                     	movups %xmm3,0x68(%rdx)
  1519e1:	0f 28 84 24 a0 00 00 00                         	movaps 0xa0(%rsp),%xmm0
  1519e9:	0f 11 42 78                                     	movups %xmm0,0x78(%rdx)
  1519ed:	0f 28 84 24 b0 00 00 00                         	movaps 0xb0(%rsp),%xmm0
  1519f5:	0f 11 82 88 00 00 00                            	movups %xmm0,0x88(%rdx)
  1519fc:	0f 28 84 24 c0 00 00 00                         	movaps 0xc0(%rsp),%xmm0
  151a04:	0f 11 82 98 00 00 00                            	movups %xmm0,0x98(%rdx)
  151a0b:	8b 04 24                                        	mov    (%rsp),%eax
  151a0e:	8b 4c 24 03                                     	mov    0x3(%rsp),%ecx
  151a12:	89 8a ac 00 00 00                               	mov    %ecx,0xac(%rdx)
  151a18:	89 82 a9 00 00 00                               	mov    %eax,0xa9(%rdx)
  151a1e:	4c 89 22                                        	mov    %r12,(%rdx)
  151a21:	4c 89 7a 30                                     	mov    %r15,0x30(%rdx)
  151a25:	88 9a a8 00 00 00                               	mov    %bl,0xa8(%rdx)
  151a2b:	e9 21 fc ff ff                                  	jmp    151651 <emuella_j2k_codestream::scalable_lossless::prepare_forward53+0x51>
  151a30:	48 89 c3                                        	mov    %rax,%rbx
  151a33:	48 83 7c 24 10 00                               	cmpq   $0x0,0x10(%rsp)
  151a39:	74 0b                                           	je     151a46 <emuella_j2k_codestream::scalable_lossless::prepare_forward53+0x446>
  151a3b:	48 8b 7c 24 18                                  	mov    0x18(%rsp),%rdi
  151a40:	ff 15 82 33 12 00                               	call   *0x123382(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  151a46:	48 89 df                                        	mov    %rbx,%rdi
  151a49:	e8 52 a7 11 00                                  	call   26c1a0 <_Unwind_Resume@plt>
