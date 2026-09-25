Disassembly of section .text:

00000000001b8ba0 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>>:
  1b8ba0:	55                                              	push   %rbp
  1b8ba1:	41 57                                           	push   %r15
  1b8ba3:	41 56                                           	push   %r14
  1b8ba5:	41 55                                           	push   %r13
  1b8ba7:	41 54                                           	push   %r12
  1b8ba9:	53                                              	push   %rbx
  1b8baa:	48 81 ec a8 00 00 00                            	sub    $0xa8,%rsp
  1b8bb1:	48 89 7c 24 18                                  	mov    %rdi,0x18(%rsp)
  1b8bb6:	48 8b 47 40                                     	mov    0x40(%rdi),%rax
  1b8bba:	48 89 44 24 38                                  	mov    %rax,0x38(%rsp)
  1b8bbf:	48 85 c0                                        	test   %rax,%rax
  1b8bc2:	0f 84 e2 04 00 00                               	je     1b90aa <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x50a>
  1b8bc8:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
  1b8bcd:	48 8b 40 30                                     	mov    0x30(%rax),%rax
  1b8bd1:	48 89 44 24 60                                  	mov    %rax,0x60(%rsp)
  1b8bd6:	48 85 c0                                        	test   %rax,%rax
  1b8bd9:	0f 84 cb 04 00 00                               	je     1b90aa <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x50a>
  1b8bdf:	48 89 f3                                        	mov    %rsi,%rbx
  1b8be2:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
  1b8be7:	0f b6 41 48                                     	movzbl 0x48(%rcx),%eax
  1b8beb:	83 e0 1f                                        	and    $0x1f,%eax
  1b8bee:	89 44 24 30                                     	mov    %eax,0x30(%rsp)
  1b8bf2:	0f b6 41 49                                     	movzbl 0x49(%rcx),%eax
  1b8bf6:	48 8d 04 c0                                     	lea    (%rax,%rax,8),%rax
  1b8bfa:	48 8d 04 80                                     	lea    (%rax,%rax,4),%rax
  1b8bfe:	48 8d 15 bf 93 e6 ff                            	lea    -0x196c41(%rip),%rdx        # 21fc4 <anon.bd84da5dcd26c1be380e6d835091d5c8.1255.llvm.4847318650076708598+0x4724>
  1b8c05:	48 01 c2                                        	add    %rax,%rdx
  1b8c08:	48 89 94 24 90 00 00 00                         	mov    %rdx,0x90(%rsp)
  1b8c10:	4c 8b 79 38                                     	mov    0x38(%rcx),%r15
  1b8c14:	4c 8b 21                                        	mov    (%rcx),%r12
  1b8c17:	4c 8b 71 08                                     	mov    0x8(%rcx),%r14
  1b8c1b:	48 8b 41 28                                     	mov    0x28(%rcx),%rax
  1b8c1f:	48 89 44 24 50                                  	mov    %rax,0x50(%rsp)
  1b8c24:	48 8b 41 20                                     	mov    0x20(%rcx),%rax
  1b8c28:	48 89 84 24 88 00 00 00                         	mov    %rax,0x88(%rsp)
  1b8c30:	48 8b 4c 24 38                                  	mov    0x38(%rsp),%rcx
  1b8c35:	48 89 ca                                        	mov    %rcx,%rdx
  1b8c38:	48 c1 ea 02                                     	shr    $0x2,%rdx
  1b8c3c:	89 c8                                           	mov    %ecx,%eax
  1b8c3e:	83 e0 03                                        	and    $0x3,%eax
  1b8c41:	48 83 f8 01                                     	cmp    $0x1,%rax
  1b8c45:	48 83 da ff                                     	sbb    $0xffffffffffffffff,%rdx
  1b8c49:	48 89 54 24 40                                  	mov    %rdx,0x40(%rsp)
  1b8c4e:	31 d2                                           	xor    %edx,%edx
  1b8c50:	4c 89 b4 24 80 00 00 00                         	mov    %r14,0x80(%rsp)
  1b8c58:	4c 89 7c 24 78                                  	mov    %r15,0x78(%rsp)
  1b8c5d:	4c 89 64 24 70                                  	mov    %r12,0x70(%rsp)
  1b8c62:	eb 34                                           	jmp    1b8c98 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0xf8>
  1b8c64:	66 66 66 2e 0f 1f 84 00 00 00 00 00             	data16 data16 cs nopw 0x0(%rax,%rax,1)
  1b8c70:	48 8b 54 24 48                                  	mov    0x48(%rsp),%rdx
  1b8c75:	48 83 c2 04                                     	add    $0x4,%rdx
  1b8c79:	48 8b 44 24 40                                  	mov    0x40(%rsp),%rax
  1b8c7e:	48 ff c8                                        	dec    %rax
  1b8c81:	48 8b 4c 24 58                                  	mov    0x58(%rsp),%rcx
  1b8c86:	48 83 c1 fc                                     	add    $0xfffffffffffffffc,%rcx
  1b8c8a:	48 89 44 24 40                                  	mov    %rax,0x40(%rsp)
  1b8c8f:	48 85 c0                                        	test   %rax,%rax
  1b8c92:	0f 84 12 04 00 00                               	je     1b90aa <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x50a>
  1b8c98:	48 83 f9 01                                     	cmp    $0x1,%rcx
  1b8c9c:	48 89 4c 24 58                                  	mov    %rcx,0x58(%rsp)
  1b8ca1:	48 89 cd                                        	mov    %rcx,%rbp
  1b8ca4:	48 83 d5 00                                     	adc    $0x0,%rbp
  1b8ca8:	48 83 fd 04                                     	cmp    $0x4,%rbp
  1b8cac:	b8 04 00 00 00                                  	mov    $0x4,%eax
  1b8cb1:	48 0f 43 e8                                     	cmovae %rax,%rbp
  1b8cb5:	48 89 54 24 48                                  	mov    %rdx,0x48(%rsp)
  1b8cba:	48 39 54 24 38                                  	cmp    %rdx,0x38(%rsp)
  1b8cbf:	74 af                                           	je     1b8c70 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0xd0>
  1b8cc1:	48 8b 44 24 48                                  	mov    0x48(%rsp),%rax
  1b8cc6:	48 83 c8 01                                     	or     $0x1,%rax
  1b8cca:	49 0f af c7                                     	imul   %r15,%rax
  1b8cce:	48 ff c0                                        	inc    %rax
  1b8cd1:	48 89 44 24 68                                  	mov    %rax,0x68(%rsp)
  1b8cd6:	31 c9                                           	xor    %ecx,%ecx
  1b8cd8:	48 89 ac 24 98 00 00 00                         	mov    %rbp,0x98(%rsp)
  1b8ce0:	eb 19                                           	jmp    1b8cfb <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x15b>
  1b8ce2:	66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00       	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  1b8cf0:	48 3b 4c 24 60                                  	cmp    0x60(%rsp),%rcx
  1b8cf5:	0f 84 75 ff ff ff                               	je     1b8c70 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0xd0>
  1b8cfb:	48 8b 44 24 68                                  	mov    0x68(%rsp),%rax
  1b8d00:	48 01 c8                                        	add    %rcx,%rax
  1b8d03:	48 ff c1                                        	inc    %rcx
  1b8d06:	31 d2                                           	xor    %edx,%edx
  1b8d08:	48 89 4c 24 20                                  	mov    %rcx,0x20(%rsp)
  1b8d0d:	eb 0c                                           	jmp    1b8d1b <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x17b>
  1b8d0f:	90                                              	nop
  1b8d10:	48 ff c2                                        	inc    %rdx
  1b8d13:	4c 01 f8                                        	add    %r15,%rax
  1b8d16:	48 39 ea                                        	cmp    %rbp,%rdx
  1b8d19:	74 d5                                           	je     1b8cf0 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x150>
  1b8d1b:	4c 39 f0                                        	cmp    %r14,%rax
  1b8d1e:	0f 83 98 03 00 00                               	jae    1b90bc <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x51c>
  1b8d24:	48 8d 34 40                                     	lea    (%rax,%rax,2),%rsi
  1b8d28:	48 89 b4 24 a0 00 00 00                         	mov    %rsi,0xa0(%rsp)
  1b8d30:	41 80 3c 34 00                                  	cmpb   $0x0,(%r12,%rsi,1)
  1b8d35:	75 d9                                           	jne    1b8d10 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x170>
  1b8d37:	4c 89 e7                                        	mov    %r12,%rdi
  1b8d3a:	4c 89 f6                                        	mov    %r14,%rsi
  1b8d3d:	48 89 54 24 08                                  	mov    %rdx,0x8(%rsp)
  1b8d42:	48 89 c2                                        	mov    %rax,%rdx
  1b8d45:	4c 89 f9                                        	mov    %r15,%rcx
  1b8d48:	48 89 44 24 28                                  	mov    %rax,0x28(%rsp)
  1b8d4d:	e8 7e 65 ff ff                                  	call   1af2d0 <emuella_j2k_tier1::neighborhood_at::<false>>
  1b8d52:	89 c1                                           	mov    %eax,%ecx
  1b8d54:	c1 e9 18                                        	shr    $0x18,%ecx
  1b8d57:	80 e1 01                                        	and    $0x1,%cl
  1b8d5a:	48 89 c2                                        	mov    %rax,%rdx
  1b8d5d:	48 c1 ea 20                                     	shr    $0x20,%rdx
  1b8d61:	80 e2 01                                        	and    $0x1,%dl
  1b8d64:	48 0f ba e0 30                                  	bt     $0x30,%rax
  1b8d69:	10 ca                                           	adc    %cl,%dl
  1b8d6b:	0f ba e0 08                                     	bt     $0x8,%eax
  1b8d6f:	80 d2 00                                        	adc    $0x0,%dl
  1b8d72:	48 89 c1                                        	mov    %rax,%rcx
  1b8d75:	48 c1 e9 38                                     	shr    $0x38,%rcx
  1b8d79:	89 c6                                           	mov    %eax,%esi
  1b8d7b:	40 80 e6 01                                     	and    $0x1,%sil
  1b8d7f:	89 c7                                           	mov    %eax,%edi
  1b8d81:	c1 ef 10                                        	shr    $0x10,%edi
  1b8d84:	40 80 e7 01                                     	and    $0x1,%dil
  1b8d88:	40 00 f1                                        	add    %sil,%cl
  1b8d8b:	48 0f ba e0 28                                  	bt     $0x28,%rax
  1b8d90:	48 8b 44 24 28                                  	mov    0x28(%rsp),%rax
  1b8d95:	40 10 f9                                        	adc    %dil,%cl
  1b8d98:	08 d1                                           	or     %dl,%cl
  1b8d9a:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1b8d9f:	48 8b 4c 24 20                                  	mov    0x20(%rsp),%rcx
  1b8da4:	0f 84 66 ff ff ff                               	je     1b8d10 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x170>
  1b8daa:	48 3b 44 24 50                                  	cmp    0x50(%rsp),%rax
  1b8daf:	0f 83 41 03 00 00                               	jae    1b90f6 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x556>
  1b8db5:	48 89 c2                                        	mov    %rax,%rdx
  1b8db8:	48 8b 84 24 88 00 00 00                         	mov    0x88(%rsp),%rax
  1b8dc0:	8b 04 90                                        	mov    (%rax,%rdx,4),%eax
  1b8dc3:	45 31 ed                                        	xor    %r13d,%r13d
  1b8dc6:	8b 4c 24 30                                     	mov    0x30(%rsp),%ecx
  1b8dca:	0f a3 c8                                        	bt     %ecx,%eax
  1b8dcd:	40 0f 92 c5                                     	setb   %bpl
  1b8dd1:	4c 89 e7                                        	mov    %r12,%rdi
  1b8dd4:	4c 89 f6                                        	mov    %r14,%rsi
  1b8dd7:	4c 89 f9                                        	mov    %r15,%rcx
  1b8dda:	e8 f1 64 ff ff                                  	call   1af2d0 <emuella_j2k_tier1::neighborhood_at::<false>>
  1b8ddf:	89 c1                                           	mov    %eax,%ecx
  1b8de1:	c1 e9 18                                        	shr    $0x18,%ecx
  1b8de4:	83 e1 01                                        	and    $0x1,%ecx
  1b8de7:	89 c2                                           	mov    %eax,%edx
  1b8de9:	c1 ea 08                                        	shr    $0x8,%edx
  1b8dec:	83 e2 01                                        	and    $0x1,%edx
  1b8def:	48 0f ba e0 30                                  	bt     $0x30,%rax
  1b8df4:	48 83 d2 00                                     	adc    $0x0,%rdx
  1b8df8:	89 c6                                           	mov    %eax,%esi
  1b8dfa:	83 e6 01                                        	and    $0x1,%esi
  1b8dfd:	89 c7                                           	mov    %eax,%edi
  1b8dff:	c1 ef 10                                        	shr    $0x10,%edi
  1b8e02:	83 e7 01                                        	and    $0x1,%edi
  1b8e05:	49 89 c0                                        	mov    %rax,%r8
  1b8e08:	49 c1 e8 38                                     	shr    $0x38,%r8
  1b8e0c:	49 01 f0                                        	add    %rsi,%r8
  1b8e0f:	48 0f ba e0 28                                  	bt     $0x28,%rax
  1b8e14:	49 11 f8                                        	adc    %rdi,%r8
  1b8e17:	48 8d 14 52                                     	lea    (%rdx,%rdx,2),%rdx
  1b8e1b:	48 0f ba e0 20                                  	bt     $0x20,%rax
  1b8e20:	48 13 8c 24 90 00 00 00                         	adc    0x90(%rsp),%rcx
  1b8e28:	4b 8d 04 c0                                     	lea    (%r8,%r8,8),%rax
  1b8e2c:	48 01 d1                                        	add    %rdx,%rcx
  1b8e2f:	0f b6 3c 01                                     	movzbl (%rcx,%rax,1),%edi
  1b8e33:	48 83 ff 12                                     	cmp    $0x12,%rdi
  1b8e37:	0f 87 92 02 00 00                               	ja     1b90cf <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x52f>
  1b8e3d:	0f b6 44 7b 1c                                  	movzbl 0x1c(%rbx,%rdi,2),%eax
  1b8e42:	48 83 f8 2e                                     	cmp    $0x2e,%rax
  1b8e46:	0f 87 95 02 00 00                               	ja     1b90e1 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x541>
  1b8e4c:	41 88 ed                                        	mov    %bpl,%r13b
  1b8e4f:	0f b6 4c 7b 1d                                  	movzbl 0x1d(%rbx,%rdi,2),%ecx
  1b8e54:	4c 8d 05 15 97 e6 ff                            	lea    -0x1968eb(%rip),%r8        # 22570 <anon.c3db339937c4b26e029c5c277d8a0513.101.llvm.11782617808337995929+0x31>
  1b8e5b:	41 8b 2c c0                                     	mov    (%r8,%rax,8),%ebp
  1b8e5f:	41 0f b6 54 c0 04                               	movzbl 0x4(%r8,%rax,8),%edx
  1b8e65:	41 0f b6 74 c0 05                               	movzbl 0x5(%r8,%rax,8),%esi
  1b8e6b:	45 0f b6 44 c0 06                               	movzbl 0x6(%r8,%rax,8),%r8d
  1b8e71:	8b 43 10                                        	mov    0x10(%rbx),%eax
  1b8e74:	29 e8                                           	sub    %ebp,%eax
  1b8e76:	89 43 10                                        	mov    %eax,0x10(%rbx)
  1b8e79:	41 39 cd                                        	cmp    %ecx,%r13d
  1b8e7c:	44 89 6c 24 34                                  	mov    %r13d,0x34(%rsp)
  1b8e81:	75 10                                           	jne    1b8e93 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x2f3>
  1b8e83:	66 85 c0                                        	test   %ax,%ax
  1b8e86:	78 25                                           	js     1b8ead <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x30d>
  1b8e88:	39 e8                                           	cmp    %ebp,%eax
  1b8e8a:	73 51                                           	jae    1b8edd <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x33d>
  1b8e8c:	89 6b 10                                        	mov    %ebp,0x10(%rbx)
  1b8e8f:	89 e8                                           	mov    %ebp,%eax
  1b8e91:	eb 4d                                           	jmp    1b8ee0 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x340>
  1b8e93:	39 e8                                           	cmp    %ebp,%eax
  1b8e95:	73 23                                           	jae    1b8eba <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x31a>
  1b8e97:	01 6b 14                                        	add    %ebp,0x14(%rbx)
  1b8e9a:	89 c5                                           	mov    %eax,%ebp
  1b8e9c:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1b8ea1:	40 88 74 7b 1c                                  	mov    %sil,0x1c(%rbx,%rdi,2)
  1b8ea6:	45 84 c0                                        	test   %r8b,%r8b
  1b8ea9:	75 21                                           	jne    1b8ecc <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x32c>
  1b8eab:	eb 26                                           	jmp    1b8ed3 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x333>
  1b8ead:	01 6b 14                                        	add    %ebp,0x14(%rbx)
  1b8eb0:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1b8eb5:	e9 86 01 00 00                                  	jmp    1b9040 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x4a0>
  1b8eba:	89 6b 10                                        	mov    %ebp,0x10(%rbx)
  1b8ebd:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1b8ec2:	40 88 74 7b 1c                                  	mov    %sil,0x1c(%rbx,%rdi,2)
  1b8ec7:	45 84 c0                                        	test   %r8b,%r8b
  1b8eca:	74 07                                           	je     1b8ed3 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x333>
  1b8ecc:	80 f1 01                                        	xor    $0x1,%cl
  1b8ecf:	88 4c 7b 1d                                     	mov    %cl,0x1d(%rbx,%rdi,2)
  1b8ed3:	66 85 ed                                        	test   %bp,%bp
  1b8ed6:	79 1c                                           	jns    1b8ef4 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x354>
  1b8ed8:	e9 63 01 00 00                                  	jmp    1b9040 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x4a0>
  1b8edd:	01 6b 14                                        	add    %ebp,0x14(%rbx)
  1b8ee0:	88 54 7b 1c                                     	mov    %dl,0x1c(%rbx,%rdi,2)
  1b8ee4:	89 c5                                           	mov    %eax,%ebp
  1b8ee6:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1b8eeb:	66 85 ed                                        	test   %bp,%bp
  1b8eee:	0f 88 4c 01 00 00                               	js     1b9040 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x4a0>
  1b8ef4:	44 8b 6b 14                                     	mov    0x14(%rbx),%r13d
  1b8ef8:	8b 43 18                                        	mov    0x18(%rbx),%eax
  1b8efb:	48 8b 73 08                                     	mov    0x8(%rbx),%rsi
  1b8eff:	0f b6 4b 45                                     	movzbl 0x45(%rbx),%ecx
  1b8f03:	44 0f b6 63 46                                  	movzbl 0x46(%rbx),%r12d
  1b8f08:	48 89 74 24 10                                  	mov    %rsi,0x10(%rsp)
  1b8f0d:	eb 43                                           	jmp    1b8f52 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x3b2>
  1b8f0f:	48 8b 46 08                                     	mov    0x8(%rsi),%rax
  1b8f13:	46 88 24 30                                     	mov    %r12b,(%rax,%r14,1)
  1b8f17:	49 ff c6                                        	inc    %r14
  1b8f1a:	4c 89 76 10                                     	mov    %r14,0x10(%rsi)
  1b8f1e:	45 89 ec                                        	mov    %r13d,%r12d
  1b8f21:	41 c1 ec 13                                     	shr    $0x13,%r12d
  1b8f25:	b8 08 00 00 00                                  	mov    $0x8,%eax
  1b8f2a:	b9 fe ff 07 00                                  	mov    $0x7fffe,%ecx
  1b8f2f:	c6 43 45 01                                     	movb   $0x1,0x45(%rbx)
  1b8f33:	44 88 63 46                                     	mov    %r12b,0x46(%rbx)
  1b8f37:	41 21 cd                                        	and    %ecx,%r13d
  1b8f3a:	44 89 6b 14                                     	mov    %r13d,0x14(%rbx)
  1b8f3e:	89 43 18                                        	mov    %eax,0x18(%rbx)
  1b8f41:	b1 01                                           	mov    $0x1,%cl
  1b8f43:	f7 c5 00 40 00 00                               	test   $0x4000,%ebp
  1b8f49:	44 89 fd                                        	mov    %r15d,%ebp
  1b8f4c:	0f 85 ee 00 00 00                               	jne    1b9040 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x4a0>
  1b8f52:	44 8d 3c 6d 00 00 00 00                         	lea    0x0(,%rbp,2),%r15d
  1b8f5a:	44 89 7b 10                                     	mov    %r15d,0x10(%rbx)
  1b8f5e:	45 01 ed                                        	add    %r13d,%r13d
  1b8f61:	44 89 6b 14                                     	mov    %r13d,0x14(%rbx)
  1b8f65:	ff c8                                           	dec    %eax
  1b8f67:	89 43 18                                        	mov    %eax,0x18(%rbx)
  1b8f6a:	75 d7                                           	jne    1b8f43 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x3a3>
  1b8f6c:	c6 43 45 00                                     	movb   $0x0,0x45(%rbx)
  1b8f70:	80 f9 01                                        	cmp    $0x1,%cl
  1b8f73:	75 a9                                           	jne    1b8f1e <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x37e>
  1b8f75:	41 80 fc ff                                     	cmp    $0xff,%r12b
  1b8f79:	74 58                                           	je     1b8fd3 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x433>
  1b8f7b:	41 81 fd ff ff ff 07                            	cmp    $0x7ffffff,%r13d
  1b8f82:	0f 86 8f 00 00 00                               	jbe    1b9017 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x477>
  1b8f88:	41 fe c4                                        	inc    %r12b
  1b8f8b:	4c 8b 76 10                                     	mov    0x10(%rsi),%r14
  1b8f8f:	41 80 fc ff                                     	cmp    $0xff,%r12b
  1b8f93:	0f 85 82 00 00 00                               	jne    1b901b <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x47b>
  1b8f99:	45 89 ec                                        	mov    %r13d,%r12d
  1b8f9c:	41 81 e4 fe ff ff 07                            	and    $0x7fffffe,%r12d
  1b8fa3:	44 89 63 14                                     	mov    %r12d,0x14(%rbx)
  1b8fa7:	4c 3b 36                                        	cmp    (%rsi),%r14
  1b8faa:	75 15                                           	jne    1b8fc1 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x421>
  1b8fac:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1b8fb1:	ff 15 39 4e 0b 00                               	call   *0xb4e39(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b8fb7:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  1b8fbc:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1b8fc1:	48 8b 46 08                                     	mov    0x8(%rsi),%rax
  1b8fc5:	42 c6 04 30 ff                                  	movb   $0xff,(%rax,%r14,1)
  1b8fca:	49 ff c6                                        	inc    %r14
  1b8fcd:	4c 89 76 10                                     	mov    %r14,0x10(%rsi)
  1b8fd1:	eb 31                                           	jmp    1b9004 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x464>
  1b8fd3:	4c 8b 76 10                                     	mov    0x10(%rsi),%r14
  1b8fd7:	4c 3b 36                                        	cmp    (%rsi),%r14
  1b8fda:	75 15                                           	jne    1b8ff1 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x451>
  1b8fdc:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1b8fe1:	ff 15 09 4e 0b 00                               	call   *0xb4e09(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b8fe7:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  1b8fec:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1b8ff1:	48 8b 46 08                                     	mov    0x8(%rsi),%rax
  1b8ff5:	42 c6 04 30 ff                                  	movb   $0xff,(%rax,%r14,1)
  1b8ffa:	49 ff c6                                        	inc    %r14
  1b8ffd:	4c 89 76 10                                     	mov    %r14,0x10(%rsi)
  1b9001:	45 89 ec                                        	mov    %r13d,%r12d
  1b9004:	41 c1 ec 14                                     	shr    $0x14,%r12d
  1b9008:	b8 07 00 00 00                                  	mov    $0x7,%eax
  1b900d:	b9 fe ff 0f 00                                  	mov    $0xffffe,%ecx
  1b9012:	e9 18 ff ff ff                                  	jmp    1b8f2f <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x38f>
  1b9017:	4c 8b 76 10                                     	mov    0x10(%rsi),%r14
  1b901b:	4c 3b 36                                        	cmp    (%rsi),%r14
  1b901e:	0f 85 eb fe ff ff                               	jne    1b8f0f <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x36f>
  1b9024:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1b9029:	ff 15 c1 4d 0b 00                               	call   *0xb4dc1(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b902f:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  1b9034:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1b9039:	e9 d1 fe ff ff                                  	jmp    1b8f0f <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x36f>
  1b903e:	66 90                                           	xchg   %ax,%ax
  1b9040:	4c 8b 64 24 70                                  	mov    0x70(%rsp),%r12
  1b9045:	48 8b 84 24 a0 00 00 00                         	mov    0xa0(%rsp),%rax
  1b904d:	4c 01 e0                                        	add    %r12,%rax
  1b9050:	49 89 c5                                        	mov    %rax,%r13
  1b9053:	c6 40 01 01                                     	movb   $0x1,0x1(%rax)
  1b9057:	83 7c 24 34 00                                  	cmpl   $0x0,0x34(%rsp)
  1b905c:	4c 8b b4 24 80 00 00 00                         	mov    0x80(%rsp),%r14
  1b9064:	4c 8b 7c 24 78                                  	mov    0x78(%rsp),%r15
  1b9069:	48 8b ac 24 98 00 00 00                         	mov    0x98(%rsp),%rbp
  1b9071:	48 8b 4c 24 20                                  	mov    0x20(%rsp),%rcx
  1b9076:	48 8b 44 24 28                                  	mov    0x28(%rsp),%rax
  1b907b:	0f 84 8f fc ff ff                               	je     1b8d10 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x170>
  1b9081:	48 89 c7                                        	mov    %rax,%rdi
  1b9084:	48 8b 74 24 18                                  	mov    0x18(%rsp),%rsi
  1b9089:	48 89 da                                        	mov    %rbx,%rdx
  1b908c:	e8 8f 65 ff ff                                  	call   1af620 <emuella_j2k_tier1::encode_sign_bit_at::<false>>
  1b9091:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1b9096:	48 8b 44 24 28                                  	mov    0x28(%rsp),%rax
  1b909b:	48 8b 4c 24 20                                  	mov    0x20(%rsp),%rcx
  1b90a0:	41 c6 45 00 01                                  	movb   $0x1,0x0(%r13)
  1b90a5:	e9 66 fc ff ff                                  	jmp    1b8d10 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x170>
  1b90aa:	48 81 c4 a8 00 00 00                            	add    $0xa8,%rsp
  1b90b1:	5b                                              	pop    %rbx
  1b90b2:	41 5c                                           	pop    %r12
  1b90b4:	41 5d                                           	pop    %r13
  1b90b6:	41 5e                                           	pop    %r14
  1b90b8:	41 5f                                           	pop    %r15
  1b90ba:	5d                                              	pop    %rbp
  1b90bb:	c3                                              	ret
  1b90bc:	48 8d 15 6d 07 0b 00                            	lea    0xb076d(%rip),%rdx        # 269830 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2888>
  1b90c3:	48 89 c7                                        	mov    %rax,%rdi
  1b90c6:	4c 89 f6                                        	mov    %r14,%rsi
  1b90c9:	ff 15 d9 4c 0b 00                               	call   *0xb4cd9(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b90cf:	48 8d 15 0a 0f 0b 00                            	lea    0xb0f0a(%rip),%rdx        # 269fe0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3038>
  1b90d6:	be 13 00 00 00                                  	mov    $0x13,%esi
  1b90db:	ff 15 c7 4c 0b 00                               	call   *0xb4cc7(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b90e1:	48 8d 15 10 0f 0b 00                            	lea    0xb0f10(%rip),%rdx        # 269ff8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3050>
  1b90e8:	be 2f 00 00 00                                  	mov    $0x2f,%esi
  1b90ed:	48 89 c7                                        	mov    %rax,%rdi
  1b90f0:	ff 15 b2 4c 0b 00                               	call   *0xb4cb2(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b90f6:	48 8d 15 bb 06 0b 00                            	lea    0xb06bb(%rip),%rdx        # 2697b8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2810>
  1b90fd:	48 89 c7                                        	mov    %rax,%rdi
  1b9100:	48 8b 74 24 50                                  	mov    0x50(%rsp),%rsi
  1b9105:	ff 15 9d 4c 0b 00                               	call   *0xb4c9d(%rip)        # 26dda8 <_DYNAMIC+0x228>
