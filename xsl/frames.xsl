<?xml version="1.0" encoding="ISO-8859-1"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

<xsl:template match="frames">
	<html>
		<body>
			<xsl:apply-templates select="absnode"/>
		</body>
	</html>
</xsl:template>

<xsl:template match="absnode">
	<table cellspacing="0">
		<tr>
			<td align="center" style="background-color:#E69D26">
				<table cellspacing="0">
					<tr>
						<td align="center">
							<xsl:variable name="node" select="@node"/>
							<xsl:variable name="value" select="@value"/>
							<text>
   								<nobr>
   									<xsl:value-of select="$node"/>:<xsl:value-of select="$value"/>
   									<xsl:apply-templates select="position"/>
   								</nobr>
    						</text>
    					</td>
    				</tr>
    				<tr>
    					<xsl:apply-templates select="semtype"/>
    				</tr>
    				<tr>
    					<xsl:apply-templates select="frmframe"/>
    					<xsl:apply-templates select="frmaccess"/>
    				</tr>
 					<xsl:apply-templates select="semdeclpos"/>
    				<xsl:apply-templates select="semintvalue"/>
    			</table>
    		</td>
		</tr>
		<tr>
			<td>
				<table cellspacing="0">
					<tr>
						<xsl:for-each select="absnode|symbol">
							<td valign="top">
								<xsl:apply-templates select="."/>
							</td>
						</xsl:for-each>
					</tr>
				</table>
			</td>
		</tr>
	</table>
</xsl:template>

<xsl:template match="frmframe">
		<td align="center" valign="top" style="background-color:#F5DA69">
			<xsl:variable name="label" select="@label"/>
			<xsl:variable name="level" select="@level"/>
			<xsl:variable name="fp" select="@fp"/>
			<xsl:variable name="ra" select="@ra"/>
			<xsl:variable name="size" select="@size"/>
			FRAME: label=<xsl:value-of select="$label"/> level=<xsl:value-of select="$level"/> FP=<xsl:value-of select="$fp"/> RA=<xsl:value-of select="$ra"/> size=<xsl:value-of select="$size"/>
		</td>
</xsl:template>

<xsl:template match="frmaccess">
		<td align="center" valign="top" style="background-color:#F5DA69">
			<xsl:variable name="level" select="@level"/>
			<xsl:variable name="offset" select="@offset"/>
			ACCESS: level=<xsl:value-of select="$level"/> offset=<xsl:value-of select="$offset"/>
		</td>
</xsl:template>

<xsl:template match="semtype">
    	<td align="center" valign="top">
			<table cellspacing="0">
				<tr>
					<td align="center" style="background-color:#F5DA76">
						<xsl:variable name="type" select="@type"/>
						<xsl:variable name="value" select="@value"/>
						<xsl:value-of select="$type"/>
						<xsl:if test="$value">(<xsl:value-of select="$value"/>)</xsl:if>
					</td>
				</tr>
				<tr>
					<td align="center">
						<table cellspacing="0">
							<tr>
								<xsl:for-each select="semtype">
									<td valign="top">
										<xsl:apply-templates select="."/>
									</td>
								</xsl:for-each>
							</tr>
						</table>
					</td>
				</tr>
			</table>
		</td>
</xsl:template>

<xsl:template match="semdeclpos">
    <tr>
    	<td align="center">
			<text style="background-color:#E5CA66">
				<nobr>decl@<xsl:apply-templates select="position"/></nobr>
			</text>
		</td>
	</tr>
</xsl:template>

<xsl:template match="semintvalue">
    <tr>
    	<td align="center">
			<xsl:variable name="value" select="@value"/>
			<text style="background-color:#E5CA66">
				<nobr>value=<xsl:value-of select="$value"/></nobr>
			</text>
		</td>
	</tr>
</xsl:template>

<xsl:template match="symbol">
	<table cellspacing="0">
		<tr>
			<td>
				<xsl:variable name="token" select="@token"/>
  				<xsl:variable name="lexeme" select="@lexeme"/>
    			<text style="background-color:#F5D02B">
    				<nobr><xsl:value-of select="$token"/><xsl:if test="$lexeme">=<xsl:value-of select="$lexeme"/></xsl:if><xsl:apply-templates select="position"/></nobr>
    			</text>
    		</td>
    	</tr>
    </table>
</xsl:template>

<xsl:template match="position">
	<xsl:variable name="begLine" select="@begLine"/>
	<xsl:variable name="begColumn" select="@begColumn"/>
	<xsl:variable name="endLine" select="@endLine"/>
	<xsl:variable name="endColumn" select="@endColumn"/>
	(<xsl:value-of select="$begLine"/>.<xsl:value-of select="$begColumn"/>:<xsl:value-of select="$endLine"/>.<xsl:value-of select="$endColumn"/>)
</xsl:template>

</xsl:stylesheet>