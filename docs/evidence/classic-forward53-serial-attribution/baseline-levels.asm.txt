Disassembly of section .text:

0000000000114830 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch>:
  114830:	55                                              	push   %rbp
  114831:	41 57                                           	push   %r15
  114833:	41 56                                           	push   %r14
  114835:	41 55                                           	push   %r13
  114837:	41 54                                           	push   %r12
  114839:	53                                              	push   %rbx
  11483a:	48 81 ec 88 00 00 00                            	sub    $0x88,%rsp
  114841:	45 89 cf                                        	mov    %r9d,%r15d
  114844:	48 89 4c 24 20                                  	mov    %rcx,0x20(%rsp)
  114849:	41 8d 47 ff                                     	lea    -0x1(%r15),%eax
  11484d:	3c 04                                           	cmp    $0x4,%al
  11484f:	0f 87 da 00 00 00                               	ja     11492f <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0xff>
  114855:	4c 89 c5                                        	mov    %r8,%rbp
  114858:	48 89 3c 24                                     	mov    %rdi,(%rsp)
  11485c:	4c 8b 8c 24 d0 00 00 00                         	mov    0xd0(%rsp),%r9
  114864:	41 89 f0                                        	mov    %esi,%r8d
  114867:	89 d0                                           	mov    %edx,%eax
  114869:	89 74 24 18                                     	mov    %esi,0x18(%rsp)
  11486d:	89 f1                                           	mov    %esi,%ecx
  11486f:	d1 e9                                           	shr    $1,%ecx
  114871:	4c 89 c7                                        	mov    %r8,%rdi
  114874:	48 29 cf                                        	sub    %rcx,%rdi
  114877:	89 54 24 1c                                     	mov    %edx,0x1c(%rsp)
  11487b:	89 d1                                           	mov    %edx,%ecx
  11487d:	d1 e9                                           	shr    $1,%ecx
  11487f:	48 89 c6                                        	mov    %rax,%rsi
  114882:	48 29 ce                                        	sub    %rcx,%rsi
  114885:	4c 89 44 24 28                                  	mov    %r8,0x28(%rsp)
  11488a:	48 89 44 24 30                                  	mov    %rax,0x30(%rsp)
  11488f:	4c 89 44 24 38                                  	mov    %r8,0x38(%rsp)
  114894:	48 89 7c 24 40                                  	mov    %rdi,0x40(%rsp)
  114899:	48 89 74 24 48                                  	mov    %rsi,0x48(%rsp)
  11489e:	66 c7 44 24 50 00 00                            	movw   $0x0,0x50(%rsp)
  1148a5:	66 c7 44 24 58 01 20                            	movw   $0x2001,0x58(%rsp)
  1148ac:	49 39 c0                                        	cmp    %rax,%r8
  1148af:	4c 89 44 24 68                                  	mov    %r8,0x68(%rsp)
  1148b4:	49 0f 47 c0                                     	cmova  %r8,%rax
  1148b8:	48 8d 1c 40                                     	lea    (%rax,%rax,2),%rbx
  1148bc:	4d 8b 61 10                                     	mov    0x10(%r9),%r12
  1148c0:	49 89 dd                                        	mov    %rbx,%r13
  1148c3:	4d 29 e5                                        	sub    %r12,%r13
  1148c6:	0f 86 97 00 00 00                               	jbe    114963 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x133>
  1148cc:	49 8b 01                                        	mov    (%r9),%rax
  1148cf:	4c 29 e0                                        	sub    %r12,%rax
  1148d2:	4d 89 e6                                        	mov    %r12,%r14
  1148d5:	49 39 c5                                        	cmp    %rax,%r13
  1148d8:	0f 87 eb 02 00 00                               	ja     114bc9 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x399>
  1148de:	4d 8b 41 08                                     	mov    0x8(%r9),%r8
  1148e2:	4b 8d 3c b0                                     	lea    (%r8,%r14,4),%rdi
  1148e6:	49 83 fd 02                                     	cmp    $0x2,%r13
  1148ea:	72 35                                           	jb     114921 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0xf1>
  1148ec:	49 f7 d4                                        	not    %r12
  1148ef:	4c 01 e3                                        	add    %r12,%rbx
  1148f2:	48 c1 e3 02                                     	shl    $0x2,%rbx
  1148f6:	31 f6                                           	xor    %esi,%esi
  1148f8:	48 89 da                                        	mov    %rbx,%rdx
  1148fb:	4c 89 c3                                        	mov    %r8,%rbx
  1148fe:	ff 15 54 96 15 00                               	call   *0x159654(%rip)        # 26df58 <memset@GLIBC_2.2.5>
  114904:	49 89 d8                                        	mov    %rbx,%r8
  114907:	4c 8b 8c 24 d0 00 00 00                         	mov    0xd0(%rsp),%r9
  11490f:	4b 8d 04 2e                                     	lea    (%r14,%r13,1),%rax
  114913:	48 8d 3c 83                                     	lea    (%rbx,%rax,4),%rdi
  114917:	48 83 c7 fc                                     	add    $0xfffffffffffffffc,%rdi
  11491b:	4d 01 ee                                        	add    %r13,%r14
  11491e:	49 ff ce                                        	dec    %r14
  114921:	c7 07 00 00 00 00                               	movl   $0x0,(%rdi)
  114927:	49 ff c6                                        	inc    %r14
  11492a:	4c 89 f3                                        	mov    %r14,%rbx
  11492d:	eb 38                                           	jmp    114967 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x137>
  11492f:	48 b8 02 00 00 00 00 00 00 80                   	movabs $0x8000000000000002,%rax
  114939:	48 89 07                                        	mov    %rax,(%rdi)
  11493c:	48 c7 47 08 00 00 00 00                         	movq   $0x0,0x8(%rdi)
  114944:	48 8d 05 42 47 f0 ff                            	lea    -0xfb8be(%rip),%rax        # 1908d <crossbeam_epoch::guard::unprotected::UNPROTECTED+0x674d>
  11494b:	48 89 47 18                                     	mov    %rax,0x18(%rdi)
  11494f:	48 c7 47 20 46 00 00 00                         	movq   $0x46,0x20(%rdi)
  114957:	66 c7 47 28 05 00                               	movw   $0x5,0x28(%rdi)
  11495d:	c6 47 2c 0c                                     	movb   $0xc,0x2c(%rdi)
  114961:	eb 7d                                           	jmp    1149e0 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x1b0>
  114963:	4d 8b 41 08                                     	mov    0x8(%r9),%r8
  114967:	49 89 59 10                                     	mov    %rbx,0x10(%r9)
  11496b:	48 8d 7c 24 70                                  	lea    0x70(%rsp),%rdi
  114970:	48 8d 4c 24 28                                  	lea    0x28(%rsp),%rcx
  114975:	48 8b 74 24 20                                  	mov    0x20(%rsp),%rsi
  11497a:	48 89 ea                                        	mov    %rbp,%rdx
  11497d:	4c 89 44 24 08                                  	mov    %r8,0x8(%rsp)
  114982:	49 89 d9                                        	mov    %rbx,%r9
  114985:	ff 15 8d 9b 15 00                               	call   *0x159b8d(%rip)        # 26e518 <_DYNAMIC+0x998>
  11498b:	83 7c 24 70 ff                                  	cmpl   $0xffffffff,0x70(%rsp)
  114990:	74 3d                                           	je     1149cf <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x19f>
  114992:	48 8b 84 24 c8 00 00 00                         	mov    0xc8(%rsp),%rax
  11499a:	48 8b 8c 24 c0 00 00 00                         	mov    0xc0(%rsp),%rcx
  1149a2:	48 ba 02 00 00 00 00 00 00 80                   	movabs $0x8000000000000002,%rdx
  1149ac:	48 8b 34 24                                     	mov    (%rsp),%rsi
  1149b0:	48 89 16                                        	mov    %rdx,(%rsi)
  1149b3:	48 c7 46 08 00 00 00 00                         	movq   $0x0,0x8(%rsi)
  1149bb:	48 89 4e 18                                     	mov    %rcx,0x18(%rsi)
  1149bf:	48 89 46 20                                     	mov    %rax,0x20(%rsi)
  1149c3:	66 c7 46 28 05 00                               	movw   $0x5,0x28(%rsi)
  1149c9:	c6 46 2c 0c                                     	movb   $0xc,0x2c(%rsi)
  1149cd:	eb 11                                           	jmp    1149e0 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x1b0>
  1149cf:	41 80 ff 01                                     	cmp    $0x1,%r15b
  1149d3:	75 1d                                           	jne    1149f2 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x1c2>
  1149d5:	48 8b 04 24                                     	mov    (%rsp),%rax
  1149d9:	48 c7 00 ff ff ff ff                            	movq   $0xffffffffffffffff,(%rax)
  1149e0:	48 81 c4 88 00 00 00                            	add    $0x88,%rsp
  1149e7:	5b                                              	pop    %rbx
  1149e8:	41 5c                                           	pop    %r12
  1149ea:	41 5d                                           	pop    %r13
  1149ec:	41 5e                                           	pop    %r14
  1149ee:	41 5f                                           	pop    %r15
  1149f0:	5d                                              	pop    %rbp
  1149f1:	c3                                              	ret
  1149f2:	41 0f b6 c7                                     	movzbl %r15b,%eax
  1149f6:	83 c0 fe                                        	add    $0xfffffffe,%eax
  1149f9:	89 44 24 14                                     	mov    %eax,0x14(%rsp)
  1149fd:	45 31 e4                                        	xor    %r12d,%r12d
  114a00:	48 89 6c 24 60                                  	mov    %rbp,0x60(%rsp)
  114a05:	66 66 2e 0f 1f 84 00 00 00 00 00                	data16 cs nopw 0x0(%rax,%rax,1)
  114a10:	41 8d 4c 24 01                                  	lea    0x1(%r12),%ecx
  114a15:	8b 44 24 18                                     	mov    0x18(%rsp),%eax
  114a19:	41 89 c6                                        	mov    %eax,%r14d
  114a1c:	41 d3 ee                                        	shr    %cl,%r14d
  114a1f:	ba ff ff ff ff                                  	mov    $0xffffffff,%edx
  114a24:	d3 e2                                           	shl    %cl,%edx
  114a26:	f7 d2                                           	not    %edx
  114a28:	21 d0                                           	and    %edx,%eax
  114a2a:	83 f8 01                                        	cmp    $0x1,%eax
  114a2d:	41 83 de ff                                     	sbb    $0xffffffff,%r14d
  114a31:	8b 74 24 1c                                     	mov    0x1c(%rsp),%esi
  114a35:	89 f0                                           	mov    %esi,%eax
  114a37:	d3 e8                                           	shr    %cl,%eax
  114a39:	21 f2                                           	and    %esi,%edx
  114a3b:	83 fa 01                                        	cmp    $0x1,%edx
  114a3e:	83 d8 ff                                        	sbb    $0xffffffff,%eax
  114a41:	44 89 f1                                        	mov    %r14d,%ecx
  114a44:	d1 e9                                           	shr    $1,%ecx
  114a46:	4c 89 f2                                        	mov    %r14,%rdx
  114a49:	48 29 ca                                        	sub    %rcx,%rdx
  114a4c:	89 c1                                           	mov    %eax,%ecx
  114a4e:	d1 e9                                           	shr    $1,%ecx
  114a50:	48 89 c6                                        	mov    %rax,%rsi
  114a53:	48 29 ce                                        	sub    %rcx,%rsi
  114a56:	4c 89 74 24 28                                  	mov    %r14,0x28(%rsp)
  114a5b:	48 89 44 24 30                                  	mov    %rax,0x30(%rsp)
  114a60:	48 8b 4c 24 68                                  	mov    0x68(%rsp),%rcx
  114a65:	48 89 4c 24 38                                  	mov    %rcx,0x38(%rsp)
  114a6a:	48 89 54 24 40                                  	mov    %rdx,0x40(%rsp)
  114a6f:	48 89 74 24 48                                  	mov    %rsi,0x48(%rsp)
  114a74:	66 c7 44 24 50 00 00                            	movw   $0x0,0x50(%rsp)
  114a7b:	49 39 c6                                        	cmp    %rax,%r14
  114a7e:	49 0f 47 c6                                     	cmova  %r14,%rax
  114a82:	66 c7 44 24 58 01 20                            	movw   $0x2001,0x58(%rsp)
  114a89:	4c 8d 2c 40                                     	lea    (%rax,%rax,2),%r13
  114a8d:	4d 89 ef                                        	mov    %r13,%r15
  114a90:	49 29 df                                        	sub    %rbx,%r15
  114a93:	76 7b                                           	jbe    114b10 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x2e0>
  114a95:	48 8b 94 24 d0 00 00 00                         	mov    0xd0(%rsp),%rdx
  114a9d:	48 8b 02                                        	mov    (%rdx),%rax
  114aa0:	48 29 d8                                        	sub    %rbx,%rax
  114aa3:	48 89 dd                                        	mov    %rbx,%rbp
  114aa6:	49 39 c7                                        	cmp    %rax,%r15
  114aa9:	48 8b 44 24 08                                  	mov    0x8(%rsp),%rax
  114aae:	0f 87 b5 00 00 00                               	ja     114b69 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x339>
  114ab4:	48 8d 3c a8                                     	lea    (%rax,%rbp,4),%rdi
  114ab8:	49 83 ff 02                                     	cmp    $0x2,%r15
  114abc:	72 32                                           	jb     114af0 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x2c0>
  114abe:	48 f7 d3                                        	not    %rbx
  114ac1:	49 01 dd                                        	add    %rbx,%r13
  114ac4:	49 c1 e5 02                                     	shl    $0x2,%r13
  114ac8:	31 f6                                           	xor    %esi,%esi
  114aca:	4c 89 ea                                        	mov    %r13,%rdx
  114acd:	48 89 c3                                        	mov    %rax,%rbx
  114ad0:	ff 15 82 94 15 00                               	call   *0x159482(%rip)        # 26df58 <memset@GLIBC_2.2.5>
  114ad6:	48 8b 94 24 d0 00 00 00                         	mov    0xd0(%rsp),%rdx
  114ade:	49 8d 04 2f                                     	lea    (%r15,%rbp,1),%rax
  114ae2:	48 8d 3c 83                                     	lea    (%rbx,%rax,4),%rdi
  114ae6:	48 83 c7 fc                                     	add    $0xfffffffffffffffc,%rdi
  114aea:	4c 01 fd                                        	add    %r15,%rbp
  114aed:	48 ff cd                                        	dec    %rbp
  114af0:	c7 07 00 00 00 00                               	movl   $0x0,(%rdi)
  114af6:	48 ff c5                                        	inc    %rbp
  114af9:	4c 8b 42 08                                     	mov    0x8(%rdx),%r8
  114afd:	48 89 eb                                        	mov    %rbp,%rbx
  114b00:	48 8b 6c 24 60                                  	mov    0x60(%rsp),%rbp
  114b05:	eb 19                                           	jmp    114b20 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x2f0>
  114b07:	66 0f 1f 84 00 00 00 00 00                      	nopw   0x0(%rax,%rax,1)
  114b10:	4c 89 eb                                        	mov    %r13,%rbx
  114b13:	48 8b 94 24 d0 00 00 00                         	mov    0xd0(%rsp),%rdx
  114b1b:	4c 8b 44 24 08                                  	mov    0x8(%rsp),%r8
  114b20:	48 89 5a 10                                     	mov    %rbx,0x10(%rdx)
  114b24:	48 8d 7c 24 70                                  	lea    0x70(%rsp),%rdi
  114b29:	48 8b 74 24 20                                  	mov    0x20(%rsp),%rsi
  114b2e:	48 89 ea                                        	mov    %rbp,%rdx
  114b31:	48 8d 4c 24 28                                  	lea    0x28(%rsp),%rcx
  114b36:	4c 89 44 24 08                                  	mov    %r8,0x8(%rsp)
  114b3b:	49 89 d9                                        	mov    %rbx,%r9
  114b3e:	ff 15 d4 99 15 00                               	call   *0x1599d4(%rip)        # 26e518 <_DYNAMIC+0x998>
  114b44:	83 7c 24 70 ff                                  	cmpl   $0xffffffff,0x70(%rsp)
  114b49:	0f 85 43 fe ff ff                               	jne    114992 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x162>
  114b4f:	44 39 64 24 14                                  	cmp    %r12d,0x14(%rsp)
  114b54:	0f 84 7b fe ff ff                               	je     1149d5 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x1a5>
  114b5a:	41 ff c4                                        	inc    %r12d
  114b5d:	41 83 fc 1f                                     	cmp    $0x1f,%r12d
  114b61:	0f 85 a9 fe ff ff                               	jne    114a10 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x1e0>
  114b67:	eb 3c                                           	jmp    114ba5 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x375>
  114b69:	b9 04 00 00 00                                  	mov    $0x4,%ecx
  114b6e:	41 b8 04 00 00 00                               	mov    $0x4,%r8d
  114b74:	48 89 d7                                        	mov    %rdx,%rdi
  114b77:	48 89 de                                        	mov    %rbx,%rsi
  114b7a:	4c 89 fa                                        	mov    %r15,%rdx
  114b7d:	e8 6e 59 f9 ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  114b82:	48 8b 94 24 d0 00 00 00                         	mov    0xd0(%rsp),%rdx
  114b8a:	48 8b 42 08                                     	mov    0x8(%rdx),%rax
  114b8e:	48 8b 6a 10                                     	mov    0x10(%rdx),%rbp
  114b92:	48 8d 3c a8                                     	lea    (%rax,%rbp,4),%rdi
  114b96:	49 83 ff 02                                     	cmp    $0x2,%r15
  114b9a:	0f 83 1e ff ff ff                               	jae    114abe <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x28e>
  114ba0:	e9 4b ff ff ff                                  	jmp    114af0 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x2c0>
  114ba5:	48 b8 02 00 00 00 00 00 00 80                   	movabs $0x8000000000000002,%rax
  114baf:	48 ff c0                                        	inc    %rax
  114bb2:	48 8b 0c 24                                     	mov    (%rsp),%rcx
  114bb6:	48 89 01                                        	mov    %rax,(%rcx)
  114bb9:	44 89 71 08                                     	mov    %r14d,0x8(%rcx)
  114bbd:	c7 41 0c 00 00 00 00                            	movl   $0x0,0xc(%rcx)
  114bc4:	e9 17 fe ff ff                                  	jmp    1149e0 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x1b0>
  114bc9:	b9 04 00 00 00                                  	mov    $0x4,%ecx
  114bce:	41 b8 04 00 00 00                               	mov    $0x4,%r8d
  114bd4:	4c 89 cf                                        	mov    %r9,%rdi
  114bd7:	4c 89 e6                                        	mov    %r12,%rsi
  114bda:	4c 89 ea                                        	mov    %r13,%rdx
  114bdd:	e8 0e 59 f9 ff                                  	call   aa4f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  114be2:	4c 8b 8c 24 d0 00 00 00                         	mov    0xd0(%rsp),%r9
  114bea:	4d 8b 71 10                                     	mov    0x10(%r9),%r14
  114bee:	4d 8b 41 08                                     	mov    0x8(%r9),%r8
  114bf2:	4b 8d 3c b0                                     	lea    (%r8,%r14,4),%rdi
  114bf6:	49 83 fd 02                                     	cmp    $0x2,%r13
  114bfa:	0f 83 ec fc ff ff                               	jae    1148ec <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0xbc>
  114c00:	e9 1c fd ff ff                                  	jmp    114921 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0xf1>
