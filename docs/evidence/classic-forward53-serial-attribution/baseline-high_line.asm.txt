Disassembly of section .text:

0000000000202d50 <emuella_j2k_transform::transform_line_forward_first_high_bounded>:
  202d50:	55                                              	push   %rbp
  202d51:	41 57                                           	push   %r15
  202d53:	41 56                                           	push   %r14
  202d55:	41 55                                           	push   %r13
  202d57:	41 54                                           	push   %r12
  202d59:	53                                              	push   %rbx
  202d5a:	50                                              	push   %rax
  202d5b:	49 89 fa                                        	mov    %rdi,%r10
  202d5e:	49 29 f2                                        	sub    %rsi,%r10
  202d61:	4c 39 d6                                        	cmp    %r10,%rsi
  202d64:	0f 85 6d 01 00 00                               	jne    202ed7 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x187>
  202d6a:	48 83 f9 02                                     	cmp    $0x2,%rcx
  202d6e:	0f 82 86 04 00 00                               	jb     2031fa <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x4aa>
  202d74:	49 39 f1                                        	cmp    %rsi,%r9
  202d77:	0f 86 94 04 00 00                               	jbe    203211 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x4c1>
  202d7d:	8b 02                                           	mov    (%rdx),%eax
  202d7f:	8b 7a 04                                        	mov    0x4(%rdx),%edi
  202d82:	01 ff                                           	add    %edi,%edi
  202d84:	d1 ff                                           	sar    $1,%edi
  202d86:	29 f8                                           	sub    %edi,%eax
  202d88:	41 89 04 b0                                     	mov    %eax,(%r8,%rsi,4)
  202d8c:	48 83 fe 01                                     	cmp    $0x1,%rsi
  202d90:	0f 86 78 02 00 00                               	jbe    20300e <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x2be>
  202d96:	48 8d 41 fe                                     	lea    -0x2(%rcx),%rax
  202d9a:	48 d1 e8                                        	shr    $1,%rax
  202d9d:	49 89 f2                                        	mov    %rsi,%r10
  202da0:	49 f7 d2                                        	not    %r10
  202da3:	4d 01 ca                                        	add    %r9,%r10
  202da6:	4c 39 d0                                        	cmp    %r10,%rax
  202da9:	4c 89 d7                                        	mov    %r10,%rdi
  202dac:	48 0f 42 f8                                     	cmovb  %rax,%rdi
  202db0:	48 8d 5e fe                                     	lea    -0x2(%rsi),%rbx
  202db4:	48 39 df                                        	cmp    %rbx,%rdi
  202db7:	48 0f 43 fb                                     	cmovae %rbx,%rdi
  202dbb:	41 bb 01 00 00 00                               	mov    $0x1,%r11d
  202dc1:	48 83 ff 03                                     	cmp    $0x3,%rdi
  202dc5:	76 6c                                           	jbe    202e33 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0xe3>
  202dc7:	48 ff c7                                        	inc    %rdi
  202dca:	41 89 fb                                        	mov    %edi,%r11d
  202dcd:	41 83 e3 03                                     	and    $0x3,%r11d
  202dd1:	41 be 04 00 00 00                               	mov    $0x4,%r14d
  202dd7:	4d 0f 45 f3                                     	cmovne %r11,%r14
  202ddb:	4c 29 f7                                        	sub    %r14,%rdi
  202dde:	4c 8d 5f 01                                     	lea    0x1(%rdi),%r11
  202de2:	4d 8d 34 b0                                     	lea    (%r8,%rsi,4),%r14
  202de6:	49 83 c6 04                                     	add    $0x4,%r14
  202dea:	45 31 ff                                        	xor    %r15d,%r15d
  202ded:	0f 1f 00                                        	nopl   (%rax)
  202df0:	42 0f 10 44 fa 04                               	movups 0x4(%rdx,%r15,8),%xmm0
  202df6:	42 0f 10 4c fa 08                               	movups 0x8(%rdx,%r15,8),%xmm1
  202dfc:	42 0f 10 54 fa 14                               	movups 0x14(%rdx,%r15,8),%xmm2
  202e02:	0f c6 c2 88                                     	shufps $0x88,%xmm2,%xmm0
  202e06:	42 0f 10 54 fa 18                               	movups 0x18(%rdx,%r15,8),%xmm2
  202e0c:	0f 28 d9                                        	movaps %xmm1,%xmm3
  202e0f:	0f c6 da 88                                     	shufps $0x88,%xmm2,%xmm3
  202e13:	0f c6 ca dd                                     	shufps $0xdd,%xmm2,%xmm1
  202e17:	66 0f fe c8                                     	paddd  %xmm0,%xmm1
  202e1b:	66 0f 72 e1 01                                  	psrad  $0x1,%xmm1
  202e20:	66 0f fa d9                                     	psubd  %xmm1,%xmm3
  202e24:	f3 43 0f 7f 1c be                               	movdqu %xmm3,(%r14,%r15,4)
  202e2a:	49 83 c7 04                                     	add    $0x4,%r15
  202e2e:	4c 39 ff                                        	cmp    %r15,%rdi
  202e31:	75 bd                                           	jne    202df0 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0xa0>
  202e33:	48 ff c0                                        	inc    %rax
  202e36:	4d 8d 34 b0                                     	lea    (%r8,%rsi,4),%r14
  202e3a:	49 89 f7                                        	mov    %rsi,%r15
  202e3d:	4d 29 cf                                        	sub    %r9,%r15
  202e40:	4a 8d 3c 5d 01 00 00 00                         	lea    0x1(,%r11,2),%rdi
  202e48:	0f 1f 84 00 00 00 00 00                         	nopl   0x0(%rax,%rax,1)
  202e50:	4c 39 d8                                        	cmp    %r11,%rax
  202e53:	0f 84 f1 02 00 00                               	je     20314a <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x3fa>
  202e59:	4d 89 fc                                        	mov    %r15,%r12
  202e5c:	4d 01 dc                                        	add    %r11,%r12
  202e5f:	0f 84 f5 02 00 00                               	je     20315a <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x40a>
  202e65:	4d 8d 63 01                                     	lea    0x1(%r11),%r12
  202e69:	42 8b 6c da fc                                  	mov    -0x4(%rdx,%r11,8),%ebp
  202e6e:	46 8b 2c da                                     	mov    (%rdx,%r11,8),%r13d
  202e72:	42 03 6c e2 fc                                  	add    -0x4(%rdx,%r12,8),%ebp
  202e77:	d1 fd                                           	sar    $1,%ebp
  202e79:	41 29 ed                                        	sub    %ebp,%r13d
  202e7c:	47 89 2c 9e                                     	mov    %r13d,(%r14,%r11,4)
  202e80:	48 83 c7 02                                     	add    $0x2,%rdi
  202e84:	4d 89 e3                                        	mov    %r12,%r11
  202e87:	4c 39 e6                                        	cmp    %r12,%rsi
  202e8a:	75 c4                                           	jne    202e50 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x100>
  202e8c:	49 89 cb                                        	mov    %rcx,%r11
  202e8f:	49 d1 eb                                        	shr    $1,%r11
  202e92:	4d 39 d3                                        	cmp    %r10,%r11
  202e95:	4d 89 d6                                        	mov    %r10,%r14
  202e98:	4d 0f 42 f3                                     	cmovb  %r11,%r14
  202e9c:	49 39 de                                        	cmp    %rbx,%r14
  202e9f:	4c 0f 43 f3                                     	cmovae %rbx,%r14
  202ea3:	49 83 fe 07                                     	cmp    $0x7,%r14
  202ea7:	76 26                                           	jbe    202ecf <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x17f>
  202ea9:	48 8d 04 b5 00 00 00 00                         	lea    0x0(,%rsi,4),%rax
  202eb1:	48 89 c7                                        	mov    %rax,%rdi
  202eb4:	48 f7 df                                        	neg    %rdi
  202eb7:	48 83 ff 10                                     	cmp    $0x10,%rdi
  202ebb:	40 0f 92 c7                                     	setb   %dil
  202ebf:	48 83 f8 ed                                     	cmp    $0xffffffffffffffed,%rax
  202ec3:	0f 93 c3                                        	setae  %bl
  202ec6:	40 08 fb                                        	or     %dil,%bl
  202ec9:	0f 84 48 01 00 00                               	je     203017 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x2c7>
  202ecf:	45 31 f6                                        	xor    %r14d,%r14d
  202ed2:	e9 a2 01 00 00                                  	jmp    203079 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x329>
  202ed7:	48 39 f7                                        	cmp    %rsi,%rdi
  202eda:	0f 84 b1 00 00 00                               	je     202f91 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x241>
  202ee0:	48 83 f9 02                                     	cmp    $0x2,%rcx
  202ee4:	0f 82 70 03 00 00                               	jb     20325a <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x50a>
  202eea:	4c 89 c8                                        	mov    %r9,%rax
  202eed:	48 29 f0                                        	sub    %rsi,%rax
  202ef0:	0f 86 e1 02 00 00                               	jbe    2031d7 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x487>
  202ef6:	8b 3a                                           	mov    (%rdx),%edi
  202ef8:	44 8b 5a 04                                     	mov    0x4(%rdx),%r11d
  202efc:	45 01 db                                        	add    %r11d,%r11d
  202eff:	41 d1 fb                                        	sar    $1,%r11d
  202f02:	44 29 df                                        	sub    %r11d,%edi
  202f05:	41 89 3c b0                                     	mov    %edi,(%r8,%rsi,4)
  202f09:	49 83 fa 01                                     	cmp    $0x1,%r10
  202f0d:	74 79                                           	je     202f88 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x238>
  202f0f:	4c 8d 59 01                                     	lea    0x1(%rcx),%r11
  202f13:	49 d1 eb                                        	shr    $1,%r11
  202f16:	48 89 cb                                        	mov    %rcx,%rbx
  202f19:	48 d1 eb                                        	shr    $1,%rbx
  202f1c:	4d 8d 34 b0                                     	lea    (%r8,%rsi,4),%r14
  202f20:	48 f7 db                                        	neg    %rbx
  202f23:	41 bf 01 00 00 00                               	mov    $0x1,%r15d
  202f29:	bf 03 00 00 00                                  	mov    $0x3,%edi
  202f2e:	66 90                                           	xchg   %ax,%ax
  202f30:	4e 8d 24 3b                                     	lea    (%rbx,%r15,1),%r12
  202f34:	49 83 fc 01                                     	cmp    $0x1,%r12
  202f38:	0f 84 6f 02 00 00                               	je     2031ad <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x45d>
  202f3e:	42 8b 6c fa fc                                  	mov    -0x4(%rdx,%r15,8),%ebp
  202f43:	41 89 ec                                        	mov    %ebp,%r12d
  202f46:	49 39 f7                                        	cmp    %rsi,%r15
  202f49:	73 0e                                           	jae    202f59 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x209>
  202f4b:	48 39 cf                                        	cmp    %rcx,%rdi
  202f4e:	0f 83 96 02 00 00                               	jae    2031ea <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x49a>
  202f54:	46 8b 64 fa 04                                  	mov    0x4(%rdx,%r15,8),%r12d
  202f59:	4d 39 fb                                        	cmp    %r15,%r11
  202f5c:	0f 84 5f 02 00 00                               	je     2031c1 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x471>
  202f62:	4c 39 f8                                        	cmp    %r15,%rax
  202f65:	0f 84 69 02 00 00                               	je     2031d4 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x484>
  202f6b:	46 8b 2c fa                                     	mov    (%rdx,%r15,8),%r13d
  202f6f:	41 01 ec                                        	add    %ebp,%r12d
  202f72:	41 d1 fc                                        	sar    $1,%r12d
  202f75:	45 29 e5                                        	sub    %r12d,%r13d
  202f78:	47 89 2c be                                     	mov    %r13d,(%r14,%r15,4)
  202f7c:	49 ff c7                                        	inc    %r15
  202f7f:	48 83 c7 02                                     	add    $0x2,%rdi
  202f83:	4d 39 fa                                        	cmp    %r15,%r10
  202f86:	75 a8                                           	jne    202f30 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x1e0>
  202f88:	48 85 f6                                        	test   %rsi,%rsi
  202f8b:	0f 84 87 01 00 00                               	je     203118 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x3c8>
  202f91:	49 89 cb                                        	mov    %rcx,%r11
  202f94:	49 d1 eb                                        	shr    $1,%r11
  202f97:	49 8d 1c b0                                     	lea    (%r8,%rsi,4),%rbx
  202f9b:	48 83 c3 04                                     	add    $0x4,%rbx
  202f9f:	bf 01 00 00 00                                  	mov    $0x1,%edi
  202fa4:	45 31 f6                                        	xor    %r14d,%r14d
  202fa7:	66 0f 1f 84 00 00 00 00 00                      	nopw   0x0(%rax,%rax,1)
  202fb0:	4a 8d 04 36                                     	lea    (%rsi,%r14,1),%rax
  202fb4:	4c 39 c8                                        	cmp    %r9,%rax
  202fb7:	0f 83 7a 01 00 00                               	jae    203137 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x3e7>
  202fbd:	4d 8d 7e 01                                     	lea    0x1(%r14),%r15
  202fc1:	46 8b 64 b3 fc                                  	mov    -0x4(%rbx,%r14,4),%r12d
  202fc6:	44 89 e0                                        	mov    %r12d,%eax
  202fc9:	4d 39 d7                                        	cmp    %r10,%r15
  202fcc:	73 14                                           	jae    202fe2 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x292>
  202fce:	4a 8d 04 36                                     	lea    (%rsi,%r14,1),%rax
  202fd2:	48 ff c0                                        	inc    %rax
  202fd5:	4c 39 c8                                        	cmp    %r9,%rax
  202fd8:	0f 83 bc 01 00 00                               	jae    20319a <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x44a>
  202fde:	42 8b 04 b3                                     	mov    (%rbx,%r14,4),%eax
  202fe2:	4d 39 f3                                        	cmp    %r14,%r11
  202fe5:	0f 84 3c 01 00 00                               	je     203127 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x3d7>
  202feb:	44 01 e0                                        	add    %r12d,%eax
  202fee:	83 c0 02                                        	add    $0x2,%eax
  202ff1:	c1 f8 02                                        	sar    $0x2,%eax
  202ff4:	42 03 44 f2 04                                  	add    0x4(%rdx,%r14,8),%eax
  202ff9:	43 89 04 b0                                     	mov    %eax,(%r8,%r14,4)
  202ffd:	48 83 c7 02                                     	add    $0x2,%rdi
  203001:	4d 89 fe                                        	mov    %r15,%r14
  203004:	4c 39 fe                                        	cmp    %r15,%rsi
  203007:	75 a7                                           	jne    202fb0 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x260>
  203009:	e9 0a 01 00 00                                  	jmp    203118 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x3c8>
  20300e:	48 8d 7e ff                                     	lea    -0x1(%rsi),%rdi
  203012:	e9 c6 00 00 00                                  	jmp    2030dd <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x38d>
  203017:	49 ff c6                                        	inc    %r14
  20301a:	44 89 f7                                        	mov    %r14d,%edi
  20301d:	83 e7 03                                        	and    $0x3,%edi
  203020:	bb 04 00 00 00                                  	mov    $0x4,%ebx
  203025:	48 0f 45 df                                     	cmovne %rdi,%rbx
  203029:	49 29 de                                        	sub    %rbx,%r14
  20302c:	4c 01 c0                                        	add    %r8,%rax
  20302f:	48 83 c0 04                                     	add    $0x4,%rax
  203033:	31 ff                                           	xor    %edi,%edi
  203035:	66 0f 6f 05 53 d3 e0 ff                         	movdqa -0x1f2cad(%rip),%xmm0        # 10390 <anon.ee651107ab5319c6bc273e1a29320aaf.78.llvm.14746981713632465754+0x40>
  20303d:	0f 1f 00                                        	nopl   (%rax)
  203040:	0f 10 4c fa 04                                  	movups 0x4(%rdx,%rdi,8),%xmm1
  203045:	0f 10 54 fa 14                                  	movups 0x14(%rdx,%rdi,8),%xmm2
  20304a:	0f c6 ca 88                                     	shufps $0x88,%xmm2,%xmm1
  20304e:	f3 0f 6f 54 b8 fc                               	movdqu -0x4(%rax,%rdi,4),%xmm2
  203054:	f3 0f 6f 1c b8                                  	movdqu (%rax,%rdi,4),%xmm3
  203059:	66 0f fe da                                     	paddd  %xmm2,%xmm3
  20305d:	66 0f fe d8                                     	paddd  %xmm0,%xmm3
  203061:	66 0f 72 e3 02                                  	psrad  $0x2,%xmm3
  203066:	66 0f fe d9                                     	paddd  %xmm1,%xmm3
  20306a:	f3 41 0f 7f 1c b8                               	movdqu %xmm3,(%r8,%rdi,4)
  203070:	48 83 c7 04                                     	add    $0x4,%rdi
  203074:	49 39 fe                                        	cmp    %rdi,%r14
  203077:	75 c7                                           	jne    203040 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x2f0>
  203079:	48 8d 7e ff                                     	lea    -0x1(%rsi),%rdi
  20307d:	49 8d 1c b0                                     	lea    (%r8,%rsi,4),%rbx
  203081:	48 83 c3 04                                     	add    $0x4,%rbx
  203085:	4a 8d 04 75 01 00 00 00                         	lea    0x1(,%r14,2),%rax
  20308d:	4c 8d 7e ff                                     	lea    -0x1(%rsi),%r15
  203091:	66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00    	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  2030a0:	4d 39 f2                                        	cmp    %r14,%r10
  2030a3:	0f 84 c7 00 00 00                               	je     203170 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x420>
  2030a9:	4d 39 f3                                        	cmp    %r14,%r11
  2030ac:	0f 84 d5 00 00 00                               	je     203187 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x437>
  2030b2:	4d 8d 66 01                                     	lea    0x1(%r14),%r12
  2030b6:	46 8b 6c b3 fc                                  	mov    -0x4(%rbx,%r14,4),%r13d
  2030bb:	42 8b 2c b3                                     	mov    (%rbx,%r14,4),%ebp
  2030bf:	44 01 ed                                        	add    %r13d,%ebp
  2030c2:	83 c5 02                                        	add    $0x2,%ebp
  2030c5:	c1 fd 02                                        	sar    $0x2,%ebp
  2030c8:	42 03 6c e2 fc                                  	add    -0x4(%rdx,%r12,8),%ebp
  2030cd:	43 89 2c b0                                     	mov    %ebp,(%r8,%r14,4)
  2030d1:	48 83 c0 02                                     	add    $0x2,%rax
  2030d5:	4d 89 e6                                        	mov    %r12,%r14
  2030d8:	4d 39 e7                                        	cmp    %r12,%r15
  2030db:	75 c3                                           	jne    2030a0 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x350>
  2030dd:	48 01 fe                                        	add    %rdi,%rsi
  2030e0:	4c 39 ce                                        	cmp    %r9,%rsi
  2030e3:	0f 83 3b 01 00 00                               	jae    203224 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x4d4>
  2030e9:	48 8d 04 7d 01 00 00 00                         	lea    0x1(,%rdi,2),%rax
  2030f1:	48 39 c8                                        	cmp    %rcx,%rax
  2030f4:	0f 83 3d 01 00 00                               	jae    203237 <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x4e7>
  2030fa:	4c 39 cf                                        	cmp    %r9,%rdi
  2030fd:	0f 83 47 01 00 00                               	jae    20324a <emuella_j2k_transform::transform_line_forward_first_high_bounded+0x4fa>
  203103:	41 8b 0c b0                                     	mov    (%r8,%rsi,4),%ecx
  203107:	8d 0c 4d 02 00 00 00                            	lea    0x2(,%rcx,2),%ecx
  20310e:	c1 f9 02                                        	sar    $0x2,%ecx
  203111:	03 0c 82                                        	add    (%rdx,%rax,4),%ecx
  203114:	41 89 0c b8                                     	mov    %ecx,(%r8,%rdi,4)
  203118:	48 83 c4 08                                     	add    $0x8,%rsp
  20311c:	5b                                              	pop    %rbx
  20311d:	41 5c                                           	pop    %r12
  20311f:	41 5d                                           	pop    %r13
  203121:	41 5e                                           	pop    %r14
  203123:	41 5f                                           	pop    %r15
  203125:	5d                                              	pop    %rbp
  203126:	c3                                              	ret
  203127:	48 8d 15 e2 7d 06 00                            	lea    0x67de2(%rip),%rdx        # 26af10 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3f68>
  20312e:	48 89 ce                                        	mov    %rcx,%rsi
  203131:	ff 15 71 ac 06 00                               	call   *0x6ac71(%rip)        # 26dda8 <_DYNAMIC+0x228>
  203137:	48 8d 15 a2 7d 06 00                            	lea    0x67da2(%rip),%rdx        # 26aee0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3f38>
  20313e:	48 89 c7                                        	mov    %rax,%rdi
  203141:	4c 89 ce                                        	mov    %r9,%rsi
  203144:	ff 15 5e ac 06 00                               	call   *0x6ac5e(%rip)        # 26dda8 <_DYNAMIC+0x228>
  20314a:	48 8d 15 df 81 06 00                            	lea    0x681df(%rip),%rdx        # 26b330 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x4388>
  203151:	48 89 ce                                        	mov    %rcx,%rsi
  203154:	ff 15 4e ac 06 00                               	call   *0x6ac4e(%rip)        # 26dda8 <_DYNAMIC+0x228>
  20315a:	4c 01 de                                        	add    %r11,%rsi
  20315d:	48 8d 15 e4 81 06 00                            	lea    0x681e4(%rip),%rdx        # 26b348 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x43a0>
  203164:	48 89 f7                                        	mov    %rsi,%rdi
  203167:	4c 89 ce                                        	mov    %r9,%rsi
  20316a:	ff 15 38 ac 06 00                               	call   *0x6ac38(%rip)        # 26dda8 <_DYNAMIC+0x228>
  203170:	4a 8d 3c 36                                     	lea    (%rsi,%r14,1),%rdi
  203174:	48 ff c7                                        	inc    %rdi
  203177:	48 8d 15 82 81 06 00                            	lea    0x68182(%rip),%rdx        # 26b300 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x4358>
  20317e:	4c 89 ce                                        	mov    %r9,%rsi
  203181:	ff 15 21 ac 06 00                               	call   *0x6ac21(%rip)        # 26dda8 <_DYNAMIC+0x228>
  203187:	48 8d 15 8a 81 06 00                            	lea    0x6818a(%rip),%rdx        # 26b318 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x4370>
  20318e:	48 89 c7                                        	mov    %rax,%rdi
  203191:	48 89 ce                                        	mov    %rcx,%rsi
  203194:	ff 15 0e ac 06 00                               	call   *0x6ac0e(%rip)        # 26dda8 <_DYNAMIC+0x228>
  20319a:	48 8d 15 57 7d 06 00                            	lea    0x67d57(%rip),%rdx        # 26aef8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3f50>
  2031a1:	48 89 c7                                        	mov    %rax,%rdi
  2031a4:	4c 89 ce                                        	mov    %r9,%rsi
  2031a7:	ff 15 fb ab 06 00                               	call   *0x6abfb(%rip)        # 26dda8 <_DYNAMIC+0x228>
  2031ad:	48 83 c7 fe                                     	add    $0xfffffffffffffffe,%rdi
  2031b1:	48 8d 15 88 7d 06 00                            	lea    0x67d88(%rip),%rdx        # 26af40 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3f98>
  2031b8:	48 89 ce                                        	mov    %rcx,%rsi
  2031bb:	ff 15 e7 ab 06 00                               	call   *0x6abe7(%rip)        # 26dda8 <_DYNAMIC+0x228>
  2031c1:	48 ff cf                                        	dec    %rdi
  2031c4:	48 8d 15 a5 7d 06 00                            	lea    0x67da5(%rip),%rdx        # 26af70 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3fc8>
  2031cb:	48 89 ce                                        	mov    %rcx,%rsi
  2031ce:	ff 15 d4 ab 06 00                               	call   *0x6abd4(%rip)        # 26dda8 <_DYNAMIC+0x228>
  2031d4:	4c 89 ce                                        	mov    %r9,%rsi
  2031d7:	48 8d 15 aa 7d 06 00                            	lea    0x67daa(%rip),%rdx        # 26af88 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3fe0>
  2031de:	48 89 f7                                        	mov    %rsi,%rdi
  2031e1:	4c 89 ce                                        	mov    %r9,%rsi
  2031e4:	ff 15 be ab 06 00                               	call   *0x6abbe(%rip)        # 26dda8 <_DYNAMIC+0x228>
  2031ea:	48 8d 15 67 7d 06 00                            	lea    0x67d67(%rip),%rdx        # 26af58 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3fb0>
  2031f1:	48 89 ce                                        	mov    %rcx,%rsi
  2031f4:	ff 15 ae ab 06 00                               	call   *0x6abae(%rip)        # 26dda8 <_DYNAMIC+0x228>
  2031fa:	48 8d 15 87 80 06 00                            	lea    0x68087(%rip),%rdx        # 26b288 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x42e0>
  203201:	bf 01 00 00 00                                  	mov    $0x1,%edi
  203206:	be 01 00 00 00                                  	mov    $0x1,%esi
  20320b:	ff 15 97 ab 06 00                               	call   *0x6ab97(%rip)        # 26dda8 <_DYNAMIC+0x228>
  203211:	48 8d 15 88 80 06 00                            	lea    0x68088(%rip),%rdx        # 26b2a0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x42f8>
  203218:	48 89 f7                                        	mov    %rsi,%rdi
  20321b:	4c 89 ce                                        	mov    %r9,%rsi
  20321e:	ff 15 84 ab 06 00                               	call   *0x6ab84(%rip)        # 26dda8 <_DYNAMIC+0x228>
  203224:	48 8d 15 8d 80 06 00                            	lea    0x6808d(%rip),%rdx        # 26b2b8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x4310>
  20322b:	48 89 f7                                        	mov    %rsi,%rdi
  20322e:	4c 89 ce                                        	mov    %r9,%rsi
  203231:	ff 15 71 ab 06 00                               	call   *0x6ab71(%rip)        # 26dda8 <_DYNAMIC+0x228>
  203237:	48 8d 15 92 80 06 00                            	lea    0x68092(%rip),%rdx        # 26b2d0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x4328>
  20323e:	48 89 c7                                        	mov    %rax,%rdi
  203241:	48 89 ce                                        	mov    %rcx,%rsi
  203244:	ff 15 5e ab 06 00                               	call   *0x6ab5e(%rip)        # 26dda8 <_DYNAMIC+0x228>
  20324a:	48 8d 15 97 80 06 00                            	lea    0x68097(%rip),%rdx        # 26b2e8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x4340>
  203251:	4c 89 ce                                        	mov    %r9,%rsi
  203254:	ff 15 4e ab 06 00                               	call   *0x6ab4e(%rip)        # 26dda8 <_DYNAMIC+0x228>
  20325a:	48 8d 15 c7 7c 06 00                            	lea    0x67cc7(%rip),%rdx        # 26af28 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3f80>
  203261:	bf 01 00 00 00                                  	mov    $0x1,%edi
  203266:	be 01 00 00 00                                  	mov    $0x1,%esi
  20326b:	ff 15 37 ab 06 00                               	call   *0x6ab37(%rip)        # 26dda8 <_DYNAMIC+0x228>
