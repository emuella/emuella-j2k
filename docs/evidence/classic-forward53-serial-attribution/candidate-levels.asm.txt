Disassembly of section .text:

0000000000115aa0 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch>:
  115aa0:	55                                              	push   %rbp
  115aa1:	41 57                                           	push   %r15
  115aa3:	41 56                                           	push   %r14
  115aa5:	41 55                                           	push   %r13
  115aa7:	41 54                                           	push   %r12
  115aa9:	53                                              	push   %rbx
  115aaa:	48 81 ec 88 00 00 00                            	sub    $0x88,%rsp
  115ab1:	45 89 cf                                        	mov    %r9d,%r15d
  115ab4:	48 89 4c 24 20                                  	mov    %rcx,0x20(%rsp)
  115ab9:	41 8d 47 ff                                     	lea    -0x1(%r15),%eax
  115abd:	3c 04                                           	cmp    $0x4,%al
  115abf:	0f 87 da 00 00 00                               	ja     115b9f <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0xff>
  115ac5:	4c 89 c5                                        	mov    %r8,%rbp
  115ac8:	48 89 3c 24                                     	mov    %rdi,(%rsp)
  115acc:	4c 8b 8c 24 d0 00 00 00                         	mov    0xd0(%rsp),%r9
  115ad4:	41 89 f0                                        	mov    %esi,%r8d
  115ad7:	89 d0                                           	mov    %edx,%eax
  115ad9:	89 74 24 18                                     	mov    %esi,0x18(%rsp)
  115add:	89 f1                                           	mov    %esi,%ecx
  115adf:	d1 e9                                           	shr    $1,%ecx
  115ae1:	4c 89 c7                                        	mov    %r8,%rdi
  115ae4:	48 29 cf                                        	sub    %rcx,%rdi
  115ae7:	89 54 24 1c                                     	mov    %edx,0x1c(%rsp)
  115aeb:	89 d1                                           	mov    %edx,%ecx
  115aed:	d1 e9                                           	shr    $1,%ecx
  115aef:	48 89 c6                                        	mov    %rax,%rsi
  115af2:	48 29 ce                                        	sub    %rcx,%rsi
  115af5:	4c 89 44 24 28                                  	mov    %r8,0x28(%rsp)
  115afa:	48 89 44 24 30                                  	mov    %rax,0x30(%rsp)
  115aff:	4c 89 44 24 38                                  	mov    %r8,0x38(%rsp)
  115b04:	48 89 7c 24 40                                  	mov    %rdi,0x40(%rsp)
  115b09:	48 89 74 24 48                                  	mov    %rsi,0x48(%rsp)
  115b0e:	66 c7 44 24 50 00 00                            	movw   $0x0,0x50(%rsp)
  115b15:	66 c7 44 24 58 01 20                            	movw   $0x2001,0x58(%rsp)
  115b1c:	49 39 c0                                        	cmp    %rax,%r8
  115b1f:	4c 89 44 24 68                                  	mov    %r8,0x68(%rsp)
  115b24:	49 0f 47 c0                                     	cmova  %r8,%rax
  115b28:	48 8d 1c 40                                     	lea    (%rax,%rax,2),%rbx
  115b2c:	4d 8b 61 10                                     	mov    0x10(%r9),%r12
  115b30:	49 89 dd                                        	mov    %rbx,%r13
  115b33:	4d 29 e5                                        	sub    %r12,%r13
  115b36:	0f 86 97 00 00 00                               	jbe    115bd3 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x133>
  115b3c:	49 8b 01                                        	mov    (%r9),%rax
  115b3f:	4c 29 e0                                        	sub    %r12,%rax
  115b42:	4d 89 e6                                        	mov    %r12,%r14
  115b45:	49 39 c5                                        	cmp    %rax,%r13
  115b48:	0f 87 eb 02 00 00                               	ja     115e39 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x399>
  115b4e:	4d 8b 41 08                                     	mov    0x8(%r9),%r8
  115b52:	4b 8d 3c b0                                     	lea    (%r8,%r14,4),%rdi
  115b56:	49 83 fd 02                                     	cmp    $0x2,%r13
  115b5a:	72 35                                           	jb     115b91 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0xf1>
  115b5c:	49 f7 d4                                        	not    %r12
  115b5f:	4c 01 e3                                        	add    %r12,%rbx
  115b62:	48 c1 e3 02                                     	shl    $0x2,%rbx
  115b66:	31 f6                                           	xor    %esi,%esi
  115b68:	48 89 da                                        	mov    %rbx,%rdx
  115b6b:	4c 89 c3                                        	mov    %r8,%rbx
  115b6e:	ff 15 74 f2 15 00                               	call   *0x15f274(%rip)        # 274de8 <memset@GLIBC_2.2.5>
  115b74:	49 89 d8                                        	mov    %rbx,%r8
  115b77:	4c 8b 8c 24 d0 00 00 00                         	mov    0xd0(%rsp),%r9
  115b7f:	4b 8d 04 2e                                     	lea    (%r14,%r13,1),%rax
  115b83:	48 8d 3c 83                                     	lea    (%rbx,%rax,4),%rdi
  115b87:	48 83 c7 fc                                     	add    $0xfffffffffffffffc,%rdi
  115b8b:	4d 01 ee                                        	add    %r13,%r14
  115b8e:	49 ff ce                                        	dec    %r14
  115b91:	c7 07 00 00 00 00                               	movl   $0x0,(%rdi)
  115b97:	49 ff c6                                        	inc    %r14
  115b9a:	4c 89 f3                                        	mov    %r14,%rbx
  115b9d:	eb 38                                           	jmp    115bd7 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x137>
  115b9f:	48 b8 02 00 00 00 00 00 00 80                   	movabs $0x8000000000000002,%rax
  115ba9:	48 89 07                                        	mov    %rax,(%rdi)
  115bac:	48 c7 47 08 00 00 00 00                         	movq   $0x0,0x8(%rdi)
  115bb4:	48 8d 05 f2 38 f0 ff                            	lea    -0xfc70e(%rip),%rax        # 194ad <crossbeam_epoch::guard::unprotected::UNPROTECTED+0x674d>
  115bbb:	48 89 47 18                                     	mov    %rax,0x18(%rdi)
  115bbf:	48 c7 47 20 46 00 00 00                         	movq   $0x46,0x20(%rdi)
  115bc7:	66 c7 47 28 05 00                               	movw   $0x5,0x28(%rdi)
  115bcd:	c6 47 2c 0c                                     	movb   $0xc,0x2c(%rdi)
  115bd1:	eb 7d                                           	jmp    115c50 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x1b0>
  115bd3:	4d 8b 41 08                                     	mov    0x8(%r9),%r8
  115bd7:	49 89 59 10                                     	mov    %rbx,0x10(%r9)
  115bdb:	48 8d 7c 24 70                                  	lea    0x70(%rsp),%rdi
  115be0:	48 8d 4c 24 28                                  	lea    0x28(%rsp),%rcx
  115be5:	48 8b 74 24 20                                  	mov    0x20(%rsp),%rsi
  115bea:	48 89 ea                                        	mov    %rbp,%rdx
  115bed:	4c 89 44 24 08                                  	mov    %r8,0x8(%rsp)
  115bf2:	49 89 d9                                        	mov    %rbx,%r9
  115bf5:	ff 15 55 f9 15 00                               	call   *0x15f955(%rip)        # 275550 <_DYNAMIC+0x9a0>
  115bfb:	83 7c 24 70 ff                                  	cmpl   $0xffffffff,0x70(%rsp)
  115c00:	74 3d                                           	je     115c3f <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x19f>
  115c02:	48 8b 84 24 c8 00 00 00                         	mov    0xc8(%rsp),%rax
  115c0a:	48 8b 8c 24 c0 00 00 00                         	mov    0xc0(%rsp),%rcx
  115c12:	48 ba 02 00 00 00 00 00 00 80                   	movabs $0x8000000000000002,%rdx
  115c1c:	48 8b 34 24                                     	mov    (%rsp),%rsi
  115c20:	48 89 16                                        	mov    %rdx,(%rsi)
  115c23:	48 c7 46 08 00 00 00 00                         	movq   $0x0,0x8(%rsi)
  115c2b:	48 89 4e 18                                     	mov    %rcx,0x18(%rsi)
  115c2f:	48 89 46 20                                     	mov    %rax,0x20(%rsi)
  115c33:	66 c7 46 28 05 00                               	movw   $0x5,0x28(%rsi)
  115c39:	c6 46 2c 0c                                     	movb   $0xc,0x2c(%rsi)
  115c3d:	eb 11                                           	jmp    115c50 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x1b0>
  115c3f:	41 80 ff 01                                     	cmp    $0x1,%r15b
  115c43:	75 1d                                           	jne    115c62 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x1c2>
  115c45:	48 8b 04 24                                     	mov    (%rsp),%rax
  115c49:	48 c7 00 ff ff ff ff                            	movq   $0xffffffffffffffff,(%rax)
  115c50:	48 81 c4 88 00 00 00                            	add    $0x88,%rsp
  115c57:	5b                                              	pop    %rbx
  115c58:	41 5c                                           	pop    %r12
  115c5a:	41 5d                                           	pop    %r13
  115c5c:	41 5e                                           	pop    %r14
  115c5e:	41 5f                                           	pop    %r15
  115c60:	5d                                              	pop    %rbp
  115c61:	c3                                              	ret
  115c62:	41 0f b6 c7                                     	movzbl %r15b,%eax
  115c66:	83 c0 fe                                        	add    $0xfffffffe,%eax
  115c69:	89 44 24 14                                     	mov    %eax,0x14(%rsp)
  115c6d:	45 31 e4                                        	xor    %r12d,%r12d
  115c70:	48 89 6c 24 60                                  	mov    %rbp,0x60(%rsp)
  115c75:	66 66 2e 0f 1f 84 00 00 00 00 00                	data16 cs nopw 0x0(%rax,%rax,1)
  115c80:	41 8d 4c 24 01                                  	lea    0x1(%r12),%ecx
  115c85:	8b 44 24 18                                     	mov    0x18(%rsp),%eax
  115c89:	41 89 c6                                        	mov    %eax,%r14d
  115c8c:	41 d3 ee                                        	shr    %cl,%r14d
  115c8f:	ba ff ff ff ff                                  	mov    $0xffffffff,%edx
  115c94:	d3 e2                                           	shl    %cl,%edx
  115c96:	f7 d2                                           	not    %edx
  115c98:	21 d0                                           	and    %edx,%eax
  115c9a:	83 f8 01                                        	cmp    $0x1,%eax
  115c9d:	41 83 de ff                                     	sbb    $0xffffffff,%r14d
  115ca1:	8b 74 24 1c                                     	mov    0x1c(%rsp),%esi
  115ca5:	89 f0                                           	mov    %esi,%eax
  115ca7:	d3 e8                                           	shr    %cl,%eax
  115ca9:	21 f2                                           	and    %esi,%edx
  115cab:	83 fa 01                                        	cmp    $0x1,%edx
  115cae:	83 d8 ff                                        	sbb    $0xffffffff,%eax
  115cb1:	44 89 f1                                        	mov    %r14d,%ecx
  115cb4:	d1 e9                                           	shr    $1,%ecx
  115cb6:	4c 89 f2                                        	mov    %r14,%rdx
  115cb9:	48 29 ca                                        	sub    %rcx,%rdx
  115cbc:	89 c1                                           	mov    %eax,%ecx
  115cbe:	d1 e9                                           	shr    $1,%ecx
  115cc0:	48 89 c6                                        	mov    %rax,%rsi
  115cc3:	48 29 ce                                        	sub    %rcx,%rsi
  115cc6:	4c 89 74 24 28                                  	mov    %r14,0x28(%rsp)
  115ccb:	48 89 44 24 30                                  	mov    %rax,0x30(%rsp)
  115cd0:	48 8b 4c 24 68                                  	mov    0x68(%rsp),%rcx
  115cd5:	48 89 4c 24 38                                  	mov    %rcx,0x38(%rsp)
  115cda:	48 89 54 24 40                                  	mov    %rdx,0x40(%rsp)
  115cdf:	48 89 74 24 48                                  	mov    %rsi,0x48(%rsp)
  115ce4:	66 c7 44 24 50 00 00                            	movw   $0x0,0x50(%rsp)
  115ceb:	49 39 c6                                        	cmp    %rax,%r14
  115cee:	49 0f 47 c6                                     	cmova  %r14,%rax
  115cf2:	66 c7 44 24 58 01 20                            	movw   $0x2001,0x58(%rsp)
  115cf9:	4c 8d 2c 40                                     	lea    (%rax,%rax,2),%r13
  115cfd:	4d 89 ef                                        	mov    %r13,%r15
  115d00:	49 29 df                                        	sub    %rbx,%r15
  115d03:	76 7b                                           	jbe    115d80 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x2e0>
  115d05:	48 8b 94 24 d0 00 00 00                         	mov    0xd0(%rsp),%rdx
  115d0d:	48 8b 02                                        	mov    (%rdx),%rax
  115d10:	48 29 d8                                        	sub    %rbx,%rax
  115d13:	48 89 dd                                        	mov    %rbx,%rbp
  115d16:	49 39 c7                                        	cmp    %rax,%r15
  115d19:	48 8b 44 24 08                                  	mov    0x8(%rsp),%rax
  115d1e:	0f 87 b5 00 00 00                               	ja     115dd9 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x339>
  115d24:	48 8d 3c a8                                     	lea    (%rax,%rbp,4),%rdi
  115d28:	49 83 ff 02                                     	cmp    $0x2,%r15
  115d2c:	72 32                                           	jb     115d60 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x2c0>
  115d2e:	48 f7 d3                                        	not    %rbx
  115d31:	49 01 dd                                        	add    %rbx,%r13
  115d34:	49 c1 e5 02                                     	shl    $0x2,%r13
  115d38:	31 f6                                           	xor    %esi,%esi
  115d3a:	4c 89 ea                                        	mov    %r13,%rdx
  115d3d:	48 89 c3                                        	mov    %rax,%rbx
  115d40:	ff 15 a2 f0 15 00                               	call   *0x15f0a2(%rip)        # 274de8 <memset@GLIBC_2.2.5>
  115d46:	48 8b 94 24 d0 00 00 00                         	mov    0xd0(%rsp),%rdx
  115d4e:	49 8d 04 2f                                     	lea    (%r15,%rbp,1),%rax
  115d52:	48 8d 3c 83                                     	lea    (%rbx,%rax,4),%rdi
  115d56:	48 83 c7 fc                                     	add    $0xfffffffffffffffc,%rdi
  115d5a:	4c 01 fd                                        	add    %r15,%rbp
  115d5d:	48 ff cd                                        	dec    %rbp
  115d60:	c7 07 00 00 00 00                               	movl   $0x0,(%rdi)
  115d66:	48 ff c5                                        	inc    %rbp
  115d69:	4c 8b 42 08                                     	mov    0x8(%rdx),%r8
  115d6d:	48 89 eb                                        	mov    %rbp,%rbx
  115d70:	48 8b 6c 24 60                                  	mov    0x60(%rsp),%rbp
  115d75:	eb 19                                           	jmp    115d90 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x2f0>
  115d77:	66 0f 1f 84 00 00 00 00 00                      	nopw   0x0(%rax,%rax,1)
  115d80:	4c 89 eb                                        	mov    %r13,%rbx
  115d83:	48 8b 94 24 d0 00 00 00                         	mov    0xd0(%rsp),%rdx
  115d8b:	4c 8b 44 24 08                                  	mov    0x8(%rsp),%r8
  115d90:	48 89 5a 10                                     	mov    %rbx,0x10(%rdx)
  115d94:	48 8d 7c 24 70                                  	lea    0x70(%rsp),%rdi
  115d99:	48 8b 74 24 20                                  	mov    0x20(%rsp),%rsi
  115d9e:	48 89 ea                                        	mov    %rbp,%rdx
  115da1:	48 8d 4c 24 28                                  	lea    0x28(%rsp),%rcx
  115da6:	4c 89 44 24 08                                  	mov    %r8,0x8(%rsp)
  115dab:	49 89 d9                                        	mov    %rbx,%r9
  115dae:	ff 15 9c f7 15 00                               	call   *0x15f79c(%rip)        # 275550 <_DYNAMIC+0x9a0>
  115db4:	83 7c 24 70 ff                                  	cmpl   $0xffffffff,0x70(%rsp)
  115db9:	0f 85 43 fe ff ff                               	jne    115c02 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x162>
  115dbf:	44 39 64 24 14                                  	cmp    %r12d,0x14(%rsp)
  115dc4:	0f 84 7b fe ff ff                               	je     115c45 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x1a5>
  115dca:	41 ff c4                                        	inc    %r12d
  115dcd:	41 83 fc 1f                                     	cmp    $0x1f,%r12d
  115dd1:	0f 85 a9 fe ff ff                               	jne    115c80 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x1e0>
  115dd7:	eb 3c                                           	jmp    115e15 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x375>
  115dd9:	b9 04 00 00 00                                  	mov    $0x4,%ecx
  115dde:	41 b8 04 00 00 00                               	mov    $0x4,%r8d
  115de4:	48 89 d7                                        	mov    %rdx,%rdi
  115de7:	48 89 de                                        	mov    %rbx,%rsi
  115dea:	4c 89 fa                                        	mov    %r15,%rdx
  115ded:	e8 fe 58 f9 ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  115df2:	48 8b 94 24 d0 00 00 00                         	mov    0xd0(%rsp),%rdx
  115dfa:	48 8b 42 08                                     	mov    0x8(%rdx),%rax
  115dfe:	48 8b 6a 10                                     	mov    0x10(%rdx),%rbp
  115e02:	48 8d 3c a8                                     	lea    (%rax,%rbp,4),%rdi
  115e06:	49 83 ff 02                                     	cmp    $0x2,%r15
  115e0a:	0f 83 1e ff ff ff                               	jae    115d2e <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x28e>
  115e10:	e9 4b ff ff ff                                  	jmp    115d60 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x2c0>
  115e15:	48 b8 02 00 00 00 00 00 00 80                   	movabs $0x8000000000000002,%rax
  115e1f:	48 ff c0                                        	inc    %rax
  115e22:	48 8b 0c 24                                     	mov    (%rsp),%rcx
  115e26:	48 89 01                                        	mov    %rax,(%rcx)
  115e29:	44 89 71 08                                     	mov    %r14d,0x8(%rcx)
  115e2d:	c7 41 0c 00 00 00 00                            	movl   $0x0,0xc(%rcx)
  115e34:	e9 17 fe ff ff                                  	jmp    115c50 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0x1b0>
  115e39:	b9 04 00 00 00                                  	mov    $0x4,%ecx
  115e3e:	41 b8 04 00 00 00                               	mov    $0x4,%r8d
  115e44:	4c 89 cf                                        	mov    %r9,%rdi
  115e47:	4c 89 e6                                        	mov    %r12,%rsi
  115e4a:	4c 89 ea                                        	mov    %r13,%rdx
  115e4d:	e8 9e 58 f9 ff                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  115e52:	4c 8b 8c 24 d0 00 00 00                         	mov    0xd0(%rsp),%r9
  115e5a:	4d 8b 71 10                                     	mov    0x10(%r9),%r14
  115e5e:	4d 8b 41 08                                     	mov    0x8(%r9),%r8
  115e62:	4b 8d 3c b0                                     	lea    (%r8,%r14,4),%rdi
  115e66:	49 83 fd 02                                     	cmp    $0x2,%r13
  115e6a:	0f 83 ec fc ff ff                               	jae    115b5c <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0xbc>
  115e70:	e9 1c fd ff ff                                  	jmp    115b91 <emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch+0xf1>
